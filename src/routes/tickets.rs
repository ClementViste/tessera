use crate::{
    domain::{NewTicket, TicketDescription, TicketTitle},
    helpers::error_chain_fmt,
};
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct NewTicketFormData {
    title: String,
    description: String,
}

impl TryFrom<NewTicketFormData> for NewTicket {
    type Error = String;

    /// Perform the conversion.
    fn try_from(value: NewTicketFormData) -> Result<Self, Self::Error> {
        // Parse the title.
        let title = TicketTitle::parse(value.title)?;

        // Parse the description.
        let description = TicketDescription::parse(value.description)?;

        Ok(Self { title, description })
    }
}

#[derive(thiserror::Error)]
pub enum NewTicketError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for NewTicketError {
    /// Format the value using the given formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for NewTicketError {
    /// Return appropriate status code for error.
    fn status_code(&self) -> StatusCode {
        match self {
            NewTicketError::ValidationError(_) => StatusCode::BAD_REQUEST,
            NewTicketError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Get called only if the format is `application/x-www-form-urlencoded`,
/// and the content of the request could be deserialized to the `NewTicketFormData` struct.
#[tracing::instrument(
    name = "Creating a new ticket",
    skip(pool, form),
    fields(
        ticket_title = %form.title,
        ticket_description = %form.description
    )
)]
pub async fn create_ticket(
    pool: web::Data<PgPool>,
    form: web::Form<NewTicketFormData>,
) -> Result<HttpResponse, NewTicketError> {
    // Parse the new ticket.
    let new_ticket = form.0.try_into().map_err(NewTicketError::ValidationError)?;

    // Insert the new ticket details into the tickets table.
    insert_ticket(&pool, &new_ticket)
        .await
        .context("Failed to insert the new ticket details into the tickets table")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Inserting the new ticket details into the tickets table",
    skip(pool, new_ticket)
)]
pub async fn insert_ticket(pool: &PgPool, new_ticket: &NewTicket) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO tickets (title, description)
        VALUES ($1, $2)
        "#,
        new_ticket.title.as_ref(),
        new_ticket.description.as_ref(),
    )
    .execute(pool)
    .await?;

    Ok(())
}
