use actix_web::{http::header::LOCATION, HttpResponse};

/// Returns a `303 See Other` and redirect to the specified location.
pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

/// Returns a `500 Internal Server Error` while preserving the error.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}
