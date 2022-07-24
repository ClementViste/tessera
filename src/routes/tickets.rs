use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct NewTicketFormData {
    title: String,
    description: String,
}

/// Get called only if the format is `application/x-www-form-urlencoded`,
/// and the content of the request could be deserialized to the `NewTicketFormData` struct.
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
