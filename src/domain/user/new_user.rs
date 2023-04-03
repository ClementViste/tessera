use crate::domain::user::UserUsername;
use secrecy::Secret;
use uuid::Uuid;

/// Representation of a new user.
pub struct NewUser {
    pub user_id: Uuid,
    pub username: UserUsername,
    pub password_hash: Secret<String>,
}
