/// Representation of a ticket's description.
#[derive(Debug)]
pub struct TicketDescription(String);

impl TicketDescription {
    /// Returns a valid ticket description.
    pub fn parse(s: String) -> Result<Self, String> {
        // Check if the input is empty or with a trailing whitespace-like character.
        let is_empty_or_whitespace = s.trim().is_empty();

        // Validate whether the input is a valid description or not.
        if is_empty_or_whitespace {
            Err("ticket description cannot be empty.".to_string())
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for TicketDescription {
    /// Performs the conversion.
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::TicketDescription;
    use claims::{assert_err, assert_ok};

    // Must return `Err` if the input is empty.
    #[test]
    fn ticket_description_returns_err_when_empty() {
        let name = "".to_string();
        assert_err!(TicketDescription::parse(name));
    }

    // Must return `Err` if the input is using only whitespace.
    #[test]
    fn ticket_description_returns_err_when_filled_with_whitespace() {
        let name = " ".to_string();
        assert_err!(TicketDescription::parse(name));
    }

    // Must return `Ok` if the input is valid.
    #[test]
    fn ticket_description_returns_ok_when_valid() {
        let name = "After doing ...".to_string();
        assert_ok!(TicketDescription::parse(name));
    }
}
