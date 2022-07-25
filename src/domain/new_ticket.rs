use crate::domain::{TicketDescription, TicketTitle};

pub struct NewTicket {
    pub title: TicketTitle,
    pub description: TicketDescription,
}
