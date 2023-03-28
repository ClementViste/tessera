use actix_web::{http::header::ContentType, HttpResponse};

/// Returns the homepage of the application.
pub async fn home() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=UTF-8">
                <title>Home</title>
            </head>
            <body>
                <p>Welcome to tessera!</p>
                <form action="/login" method="get">
                    <input type="submit" value="Login">
                </form>
            </body>
        </html>
        "#
        .to_string(),
    )
}
