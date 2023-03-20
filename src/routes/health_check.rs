use actix_web::HttpResponse;

/// Checks if the application is up and ready to accept incoming requests.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
