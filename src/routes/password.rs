use crate::{
    authentication::{update_password, validate_credentials, AuthError, Credentials, UserId},
    helpers::get_username,
    utils::{e500, see_other},
};
use actix_web::{
    http::header::ContentType,
    {web, HttpResponse},
};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::PgPool;
use std::fmt::Write;

/// Representation of a user's password and new passwords with form data.
#[derive(Deserialize)]
pub struct PasswordFormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

/// Returns the change password form of the application.
pub async fn change_password_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    // Get notification.
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "{}", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=UTF-8">
                    <title>Change Password</title>
                </head>
                <body>
                    {msg_html}
                    <form action="/dashboard/password" method="post">
                        <label>Current password
                            <input
                                type="password"
                                placeholder="Enter current password"
                                name="current_password"
                            >
                        </label>
                        <br>
                        <label>New password
                            <input
                                type="password"
                                placeholder="Enter new password"
                                name="new_password"
                            >
                        </label>
                        <br>
                        <label>Confirm new password
                            <input
                                type="password"
                                placeholder="Type the new password again"
                                name="new_password_check"
                            >
                        </label>
                        <br>
                        <button type="submit">Change password</button>
                    </form>
                    <p><a href="/dashboard/">&lt;- Back</a></p>
                </body>
            </html>
            "#,
        )))
}

/// Changes the password.
pub async fn change_password(
    pool: web::Data<PgPool>,
    form: web::Form<PasswordFormData>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let username = get_username(&pool, *user_id).await.map_err(e500)?;

    // Check if the two new password match.
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        // Send notification.
        FlashMessage::error(
            "You entered two different new passwords, the field values must match.",
        )
        .send();

        return Ok(see_other("/dashboard/password"));
    }

    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };

    // Check if the credentials belong to an existing user.
    if let Err(e) = validate_credentials(&pool, credentials).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                // Send notification.
                FlashMessage::error("The current password is incorrect.").send();

                Ok(see_other("/dashboard/password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    update_password(&pool, *user_id, form.0.new_password)
        .await
        .map_err(e500)?;

    // Send notification.
    FlashMessage::info("Your password has been changed.").send();

    Ok(see_other("/dashboard/password"))
}
