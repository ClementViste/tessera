use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct TicketTitle(String);

impl TicketTitle {
    /// Return a valid title.
    pub fn parse(s: String) -> Result<TicketTitle, String> {
        // Check if the input is empty or with a trailing whitespace-like character.
        let is_empty_or_whitespace = s.trim().is_empty();
        // Check if the input is too long.
        let is_too_long = s.graphemes(true).count() > 256;

        // Validate whether the input is a valid title or not.
        if is_empty_or_whitespace || is_too_long {
            Err(format!("`{}` is not a valid title.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for TicketTitle {
    /// Perform the conversion.
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::TicketTitle;
    use claim::{assert_err, assert_ok};

    // Must return `Err` if the input is using only whitespace.
    #[test]
    fn whitespace_only_title_is_rejected() {
        // Set invalid title.
        let name = " ".to_string();

        // Check.
        assert_err!(TicketTitle::parse(name));
    }

    // Must return `Err` if the input is empty.
    #[test]
    fn empty_string_is_rejected() {
        // Set invalid title.
        let name = "".to_string();

        // Check.
        assert_err!(TicketTitle::parse(name));
    }

    // Must return `Ok` if the input is not bigger than 256 characters.
    #[test]
    fn a_256_grapheme_long_title_is_valid() {
        // Set valid title.
        let name = "a".repeat(256);

        // Check.
        assert_ok!(TicketTitle::parse(name));
    }

    // Must return `Err` if the input is bigger than 256 characters.
    #[test]
    fn a_title_longer_than_256_graphemes_is_rejected() {
        // Set invalid title.
        let name = "a".repeat(257);

        // Check.
        assert_err!(TicketTitle::parse(name));
    }

    // Must return `Ok` if the input is valid.
    #[test]
    fn a_valid_title_is_parsed_successfully() {
        // Set valid title.
        let name = "Issue with ...".to_string();

        // Check.
        assert_ok!(TicketTitle::parse(name));
    }
}
