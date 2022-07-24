use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct NewTicketFormData {
    title: String,
    description: String,
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
) -> HttpResponse {
    // Insert the new ticket details into the tickets table.
    match insert_ticket(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

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
