/// Representation of a valid ticket.
#[derive(Debug, PartialEq)]
pub struct ValidTicket {
    pub id: i32,
    pub title: String,
    pub description: String,
}
