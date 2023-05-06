use crate::{
    authentication::UserId,
    domain::{NewTicket, TicketDescription, TicketTitle, ValidTicket},
    error::error_chain_fmt,
    helpers::get_username,
    utils::see_other,
};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpResponse, ResponseError,
};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use anyhow::Context;
use askama::Template;
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use std::fmt::{Debug, Write};

/// Representation of the create ticket template.
#[derive(Template)]
#[template(path = "create_ticket.html")]
struct CreateTicketTemplate {
    msg_html: String,
}

/// Representation of the see tickets template.
#[derive(Template)]
#[template(path = "see_tickets.html")]
struct SeeTicketsTemplate {
    tickets: Vec<ValidTicket>,
}

/// Representation of the see ticket template.
#[derive(Template)]
#[template(path = "see_ticket.html")]
struct SeeTicketTemplate {
    msg_html: String,
    ticket: ValidTicket,
}

/// Representation of a new ticket created with form data.
#[derive(Deserialize)]
pub struct NewTicketFormData {
    title: String,
    description: String,
    priority: String,
}

impl TryFrom<NewTicketFormData> for NewTicket {
    type Error = String;

    /// Performs the conversion.
    fn try_from(value: NewTicketFormData) -> Result<Self, Self::Error> {
        let title = TicketTitle::parse(value.title)?;
        let description = TicketDescription::parse(value.description)?;
        let priority = value.priority;

        Ok(Self {
            title,
            description,
            priority,
        })
    }
}

/// Representation of a new ticket error.
#[derive(thiserror::Error)]
pub enum TicketError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for TicketError {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for TicketError {
    /// Creates full response for error.
    fn error_response(&self) -> HttpResponse {
        let msg_html = self.to_string();

        let body = CreateTicketTemplate { msg_html }.render().unwrap();

        HttpResponse::build(self.status_code())
            .content_type(ContentType::html())
            .body(body)
    }

    /// Returns appropriate status code for error.
    fn status_code(&self) -> StatusCode {
        match self {
            TicketError::ValidationError(_) => StatusCode::BAD_REQUEST,
            TicketError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Returns the create ticket form of the application.
pub async fn create_ticket_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    // Get notification.
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "{}", m.content()).unwrap();
    }

    let body = CreateTicketTemplate { msg_html }.render().unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

/// Creates a new ticket.
#[tracing::instrument(
    name = "Creating a new ticket",
    skip(pool, form, user_id),
    fields(
        ticket_title = %form.title,
        ticket_description = %form.description,
        user_id = %&*user_id
    )
)]
pub async fn create_ticket(
    pool: web::Data<PgPool>,
    form: web::Form<NewTicketFormData>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, TicketError> {
    let new_ticket = form.0.try_into().map_err(TicketError::ValidationError)?;
    let created_by = get_username(&pool, **user_id)
        .await
        .map_err(TicketError::UnexpectedError)?;

    insert_ticket(&pool, &new_ticket, created_by)
        .await
        .context("Failed to insert the new ticket details into the tickets table")?;

    // Send notification.
    FlashMessage::info("You have successfully created a new ticket.").send();

    Ok(see_other("/dashboard/tickets/new"))
}

/// Inserts the new ticket details into the `tickets` table.
#[tracing::instrument(
    name = "Inserting the new ticket details into the tickets table",
    skip(pool, new_ticket)
)]
pub async fn insert_ticket(
    pool: &PgPool,
    new_ticket: &NewTicket,
    created_by: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO tickets (title, description, created_at, created_by, is_open, priority)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        new_ticket.title.as_ref(),
        new_ticket.description.as_ref(),
        Utc::now(),
        created_by,
        true,
        new_ticket.priority,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Sees tickets.
#[tracing::instrument(
    name = "Seeing tickets",
    skip(pool, user_id),
    fields(
        user_id=%&*user_id
    )
)]
pub async fn see_tickets(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, TicketError> {
    let tickets = get_tickets(&pool)
        .await
        .context("Failed to get the tickets details from the tickets table")?;

    let body = SeeTicketsTemplate { tickets }.render().unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

/// Return tickets.
#[tracing::instrument(name = "Getting tickets details from the tickets table", skip(pool))]
pub async fn get_tickets(pool: &PgPool) -> Result<Vec<ValidTicket>, sqlx::Error> {
    let tickets = sqlx::query_as!(ValidTicket, "SELECT * FROM tickets ORDER BY id")
        .fetch_all(pool)
        .await?;

    Ok(tickets)
}

/// Sees ticket.
#[tracing::instrument(
    name = "Seeing ticket",
    skip(pool, flash_messages, user_id, ticket_id),
    fields(
        user_id=%&*user_id,
        ticket_id=%ticket_id.0
    )
)]
pub async fn see_ticket(
    pool: web::Data<PgPool>,
    flash_messages: IncomingFlashMessages,
    user_id: web::ReqData<UserId>,
    ticket_id: web::Path<(i32,)>,
) -> Result<HttpResponse, TicketError> {
    // Get notification.
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "{}", m.content()).unwrap();
    }

    let ticket_id = ticket_id.into_inner().0;

    let ticket = get_ticket(&pool, ticket_id)
        .await
        .context("Failed to get the ticket details from the tickets table")?;

    let body = SeeTicketTemplate { msg_html, ticket }.render().unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

/// Return ticket.
#[tracing::instrument(name = "Getting ticket details from the tickets table", skip(pool, id))]
pub async fn get_ticket(pool: &PgPool, id: i32) -> Result<ValidTicket, sqlx::Error> {
    let ticket = sqlx::query_as!(ValidTicket, r#"SELECT * FROM tickets WHERE id = $1"#, id)
        .fetch_one(pool)
        .await?;

    Ok(ticket)
}

/// Closes ticket.
#[tracing::instrument(
    name = "Closing ticket",
    skip(pool, user_id, ticket_id),
    fields(
        user_id=%&*user_id,
        ticket_id=%ticket_id.0
    )
)]
pub async fn close_ticket(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
    ticket_id: web::Path<(i32,)>,
) -> Result<HttpResponse, TicketError> {
    let ticket_id = ticket_id.into_inner().0;

    update_is_open(&pool, ticket_id, false)
        .await
        .context("Failed to update the `is_open` field from the tickets table")?;

    // Send notification.
    FlashMessage::info("You have successfully closed this ticket.").send();

    let location = format!("/dashboard/tickets/{}", ticket_id);
    Ok(see_other(location.as_str()))
}

/// Updates the `is_open` field from the tickets table.
#[tracing::instrument(
    name = "Updating the `is_open` field from the tickets table",
    skip(pool, id)
)]
async fn update_is_open(pool: &PgPool, id: i32, new_status: bool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE tickets
        SET is_open = $1
        WHERE id = $2
        "#,
        new_status,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
