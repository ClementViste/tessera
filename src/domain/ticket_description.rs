#[derive(Debug)]
pub struct TicketDescription(String);

impl TicketDescription {
    /// Return a valid description.
    pub fn parse(s: String) -> Result<TicketDescription, String> {
        // Check if the input is empty or with a trailing whitespace-like character.
        let is_empty_or_whitespace = s.trim().is_empty();

        // Validate whether the input is a valid description or not.
        if is_empty_or_whitespace {
            Err(format!("`{}` is not a valid description.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for TicketDescription {
    /// Perform the conversion.
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::TicketDescription;
    use claim::{assert_err, assert_ok};

    // Must return `Err` if the input is using only whitespace.
    #[test]
    fn whitespace_only_description_is_rejected() {
        // Set invalid description.
        let name = " ".to_string();

        // Check.
        assert_err!(TicketDescription::parse(name));
    }

    // Must return `Err` if the input is empty.
    #[test]
    fn empty_string_is_rejected() {
        // Set invalid description.
        let name = "".to_string();

        // Check.
        assert_err!(TicketDescription::parse(name));
    }

    // Must return `Ok` if the input is valid.
    #[test]
    fn a_valid_description_is_parsed_successfully() {
        // Set valid description.
        let name = "After doing ...".to_string();

        // Check.
        assert_ok!(TicketDescription::parse(name));
    }
}
