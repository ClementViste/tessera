use crate::{session_state::TypedSession, utils::see_other};
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

/// Logs out the user.
pub async fn logout(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    // Log out.
    session.log_out();

    // Send notification.
    FlashMessage::info("You have successfully logged out.").send();

    Ok(see_other("/login"))
}
