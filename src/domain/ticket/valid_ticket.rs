use chrono::{DateTime, Utc};

/// Representation of a valid ticket.
#[derive(Debug, PartialEq)]
pub struct ValidTicket {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub is_open: bool,
}
