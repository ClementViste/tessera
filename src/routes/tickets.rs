use actix_web::{web, HttpResponse};
use serde::Deserialize;

/// Representation of a new ticket created with form data.
#[derive(Deserialize)]
pub struct NewTicketFormData {
    title: String,
    description: String,
}

/// Creates a new ticket.
#[tracing::instrument(
    name = "Creating a new ticket",
    skip(_form),
    fields(
        ticket_title = %_form.title,
        ticket_description = %_form.description
    )
)]
pub async fn create_ticket(_form: web::Form<NewTicketFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
