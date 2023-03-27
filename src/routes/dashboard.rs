use crate::{helpers::get_username, session_state::TypedSession, utils::e500};
use actix_web::{http::header::ContentType, web, HttpResponse};
use sqlx::PgPool;

/// Returns the dashboard of the application.
pub async fn dashboard(
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(e500)?.unwrap();
    let username = get_username(&pool, user_id).await.map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
                    <title>Dashboard</title>
                </head>
                <body>
                    <p>Welcome {username}!</p>
                    <p>Available actions:</p>
                    <ol>
                        <li><a href="/dashboard/tickets/new">Create a new ticket</a></li>
                        <li><a href="/dashboard/password">Change password</a></li>
                        <li>
                            <form name="logoutForm" action="/dashboard/logout" method="post">
                                <input type="submit" value="Logout">
                            </form>
                        </li>
                    </ol>
                </body>
            </html>
            "#,
        )))
}
