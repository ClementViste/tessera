use crate::domain::{NewTicket, TicketDescription, TicketTitle};
use actix_web::{web, HttpResponse};
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
    // Parse the new ticket.
    let new_ticket = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        }
    };

    // Insert the new ticket details into the tickets table.
    match insert_ticket(&pool, &new_ticket).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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
