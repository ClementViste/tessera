use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    error::error_chain_fmt,
    session_state::TypedSession,
};
use actix_web::{error::InternalError, http, web, HttpResponse, ResponseError};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama::Template;
use secrecy::Secret;
use serde::Deserialize;
use sqlx::PgPool;
use std::fmt::{Debug, Write};

/// Representation of the login template.
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    msg_html: String,
}

/// Representation of a user credentials with form data.
#[derive(Deserialize)]
pub struct LoginFormData {
    username: String,
    password: Secret<String>,
}

/// Representation of a login error.
#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong.")]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for LoginError {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    /// Returns appropriate status code for error.
    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::SEE_OTHER
    }
}

/// Returns the login form of the application.
pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    // Get notification.
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "{}", m.content()).unwrap();
    }

    let body = LoginTemplate { msg_html }.render().unwrap();

    HttpResponse::Ok()
        .content_type(http::header::ContentType::html())
        .body(body)
}

/// Logs in the user.
#[tracing::instrument(
    skip(pool, session, form),
    fields(
        username=tracing::field::Empty,
        user_id=tracing::field::Empty
    )
)]
pub async fn login(
    pool: web::Data<PgPool>,
    session: TypedSession,
    form: web::Form<LoginFormData>,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    // Record username.
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    // Check if the credentials belong to an existing user.
    match validate_credentials(&pool, credentials).await {
        Ok(user_id) => {
            // Record the id of the user.
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

            // Avoid session fixation attacks.
            session.renew();

            // Insert the user into the session.
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;

            Ok(HttpResponse::SeeOther()
                .insert_header((http::header::LOCATION, "/dashboard/"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };

            Err(login_redirect(e))
        }
    }
}

// Redirects to the login page with a notification as an error message.
fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    // Send notification.
    FlashMessage::error(e.to_string()).send();

    let response = HttpResponse::SeeOther()
        .insert_header((http::header::LOCATION, "/login"))
        .finish();

    InternalError::from_response(e, response)
}
