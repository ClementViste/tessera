use crate::{
    session_state::TypedSession,
    telemetry::spawn_blocking_with_tracing,
    utils::{e500, see_other},
};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    FromRequest, HttpMessage,
};
use actix_web_lab::middleware::Next;
use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::{fmt::Display, ops::Deref};
use uuid::Uuid;

/// Representation of an authentication error.
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

/// Representation of an user credentials.
pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

/// Validates credentials.
#[tracing::instrument(name = "Validating credentials", skip(pool, credentials))]
pub async fn validate_credentials(
    pool: &PgPool,
    credentials: Credentials,
) -> Result<Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    // Check if the credentials belong to an existing user.
    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(pool, &credentials.username).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }
    spawn_blocking_with_tracing(move || {
        verify_password_hash(credentials.password, expected_password_hash)
    })
    .await
    .context("Failed to spawn blocking task")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username"))
        .map_err(AuthError::InvalidCredentials)
}

/// Returns the stored credentials.
#[tracing::instrument(name = "Getting stored credentials", skip(pool, username))]
async fn get_stored_credentials(
    pool: &PgPool,
    username: &str,
) -> Result<Option<(Uuid, Secret<String>)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .context("Failed to retrieve stored credentials")?
    .map(|row| (row.user_id, Secret::new(row.password_hash)));

    Ok(row)
}

/// Verifies password hash.
#[tracing::instrument(
    name = "Verifying password hash",
    skip(password_candidate, expected_password_hash)
)]
fn verify_password_hash(
    password_candidate: Secret<String>,
    expected_password_hash: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}

/// Updates the password.
#[tracing::instrument(name = "Updating the password", skip(pool, password))]
pub async fn update_password(
    pool: &PgPool,
    user_id: Uuid,
    password: Secret<String>,
) -> Result<(), anyhow::Error> {
    let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;

    sqlx::query!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE user_id = $2
        "#,
        password_hash.expose_secret(),
        user_id
    )
    .execute(pool)
    .await
    .context("Failed to change password")?;

    Ok(())
}

/// Computes password hash.
pub fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();

    Ok(Secret::new(password_hash))
}

/// Representation of a user id.
#[derive(Clone, Copy, Debug)]
pub struct UserId(Uuid);

impl Display for UserId {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    /// Dereferences the value.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Middleware used to reject anonymous users.
pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    // Check if user has logged in.
    match session.get_user_id().map_err(e500)? {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            let response = see_other("/login");
            let e = anyhow::anyhow!("The user has not logged in");
            Err(InternalError::from_response(e, response).into())
        }
    }
}
