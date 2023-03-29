use crate::{helpers::get_username, session_state::TypedSession, utils::e500};
use actix_web::{http::header::ContentType, web, HttpResponse};
use askama::Template;
use sqlx::PgPool;

/// Representation of the dashboard template.
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    username: String,
}

/// Returns the dashboard of the application.
pub async fn dashboard(
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(e500)?.unwrap();
    let username = get_username(&pool, user_id).await.map_err(e500)?;

    let body = DashboardTemplate { username }.render().unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}
