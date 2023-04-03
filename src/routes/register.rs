use crate::{
    authentication::compute_password_hash,
    domain::{
        user::{NewUser, UserUsername},
        UserPassword,
    },
    error::error_chain_fmt,
    utils::see_other,
};
use actix_web::{
    http::{self, header::ContentType, StatusCode},
    web, HttpResponse, ResponseError,
};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use anyhow::Context;
use askama::Template;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::PgPool;
use std::fmt::{Debug, Write};
use uuid::Uuid;

/// Representation of the register template.
#[derive(Template)]
#[template(path = "register.html")]
#[allow(dead_code)]
struct RegisterTemplate {
    msg_html: String,
}

/// Representation of a user's credentials with form data.
#[derive(Deserialize)]
pub struct RegisterFormData {
    username: String,
    password: Secret<String>,
}

impl TryFrom<RegisterFormData> for NewUser {
    type Error = String;

    /// Performs the conversion.
    fn try_from(value: RegisterFormData) -> Result<Self, Self::Error> {
        let user_id = Uuid::new_v4();
        let username = UserUsername::parse(value.username)?;
        let password = UserPassword::parse(value.password.expose_secret().to_string())?;
        let password_hash = compute_password_hash(password.as_ref().to_owned().into())
            .context("Failed to hash password")
            .unwrap();

        Ok(Self {
            user_id,
            username,
            password_hash,
        })
    }
}

/// Representation of a registration error.
#[derive(thiserror::Error)]
pub enum RegisterError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for RegisterError {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for RegisterError {
    /// Creates full response for error.
    fn error_response(&self) -> HttpResponse {
        let msg_html = self.to_string();

        let body = RegisterTemplate { msg_html }.render().unwrap();

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(body)
    }

    /// Returns appropriate status code for error.
    fn status_code(&self) -> StatusCode {
        match self {
            RegisterError::ValidationError(_) => StatusCode::BAD_REQUEST,
            RegisterError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Returns the register form of the application.
pub async fn register_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    // Get notification.
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "{}", m.content()).unwrap();
    }

    let body = RegisterTemplate { msg_html }.render().unwrap();

    HttpResponse::Ok()
        .content_type(http::header::ContentType::html())
        .body(body)
}

/// Registers a new user.
#[tracing::instrument(
    name = "Registering a new user",
    skip(pool, form),
    fields(
        username = %form.0.username,
        user_id=tracing::field::Empty
    )
)]
pub async fn register(
    pool: web::Data<PgPool>,
    form: web::Form<RegisterFormData>,
) -> Result<HttpResponse, RegisterError> {
    let new_user = form.0.try_into().map_err(RegisterError::ValidationError)?;

    match insert_user(&pool, &new_user).await {
        Ok(_) => {
            // Record the id of the user.
            tracing::Span::current().record("user_id", &tracing::field::display(&new_user.user_id));

            // Send notification.
            FlashMessage::info("You have successfully registered, you can now log in.").send();

            Ok(see_other("/login"))
        }
        Err(_) => {
            // Send notification.
            FlashMessage::info("Failure to register, the username is probably already taken.")
                .send();

            Ok(see_other("/register"))
        }
    }
}

/// Inserts the new user details into the `users` table.
#[tracing::instrument(
    name = "Inserting the new user details into the users table",
    skip(pool, new_user)
)]
pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (user_id, username, password_hash)
        VALUES ($1, $2, $3)
        "#,
        new_user.user_id,
        new_user.username.as_ref(),
        new_user.password_hash.expose_secret(),
    )
    .execute(pool)
    .await?;

    Ok(())
}
