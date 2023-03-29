use actix_web::{http::header::ContentType, HttpResponse};
use askama::Template;

/// Representation of the home template.
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

/// Returns the homepage of the application.
pub async fn home() -> HttpResponse {
    let body = HomeTemplate.render().unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}
