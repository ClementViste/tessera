use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

/// Representation of a new ticket created with form data.
#[derive(Deserialize)]
pub struct NewTicketFormData {
    title: String,
    description: String,
}

/// Creates a new ticket.
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
) -> HttpResponse {
    match insert_ticket(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Inserts the new ticket details into the `tickets` table.
#[tracing::instrument(
    name = "Inserting the new ticket details into the tickets table",
    skip(pool, form)
)]
pub async fn insert_ticket(pool: &PgPool, form: &NewTicketFormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO tickets (title, description)
        VALUES ($1, $2)
        "#,
        form.title,
        form.description,
    )
    .execute(pool)
    .await?;

    Ok(())
}
