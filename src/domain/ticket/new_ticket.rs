use crate::domain::{TicketDescription, TicketTitle};

/// Representation of a new ticket.
pub struct NewTicket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub priority: String,
}
