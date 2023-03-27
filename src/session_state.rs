use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::{
    dev::Payload,
    {FromRequest, HttpRequest},
};
use std::future::{ready, Ready};
use uuid::Uuid;

/// Representation of a user's session.
pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    /// Renews the session key, assigning existing session state to new key.
    pub fn renew(&self) {
        self.0.renew();
    }

    /// Inserts the `USER_ID_KEY` key-value pair into the session.
    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    /// Returns the `USER_ID_KEY` key-value from the session.
    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }

    /// Removes session from both client and server side.
    pub fn log_out(self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    // Return same error as the implementation of `FromRequest` for `Session`.
    type Error = <Session as FromRequest>::Error;
    // Convert `TypedSession` into a `Future`.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    /// Creates a `TypedSession` from request parts asynchronously.
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
