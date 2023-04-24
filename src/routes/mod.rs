mod dashboard;
mod health_check;
mod home;
mod login;
mod logout;
mod password;
mod register;
mod tickets;

pub use dashboard::dashboard;
pub use health_check::health_check;
pub use home::home;
pub use login::{login, login_form};
pub use logout::logout;
pub use password::{change_password, change_password_form};
pub use register::{register, register_form};
pub use tickets::{
    close_ticket, create_ticket, create_ticket_form, get_ticket, get_tickets, see_ticket,
    see_tickets,
};
