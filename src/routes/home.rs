use actix_web::{http::header::ContentType, HttpResponse};

pub async fn home() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Home</title>
            </head>
            <body>
                <p>Welcome to tessera!</p>
            </body>
        </html>
        "#
        .to_string(),
    )
}
