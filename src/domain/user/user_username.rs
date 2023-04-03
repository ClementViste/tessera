use unicode_segmentation::UnicodeSegmentation;

/// Representation of a user's username.
#[derive(Debug)]
pub struct UserUsername(String);

impl UserUsername {
    /// Returns a valid username.
    pub fn parse(s: String) -> Result<Self, String> {
        // Check if the input is empty or with a trailing whitespace-like character.
        let is_empty_or_whitespace = s.trim().is_empty();
        // Check if the input is too long.
        let is_too_long = s.graphemes(true).count() > 256;

        // Validate whether the input is a valid username or not.
        if is_empty_or_whitespace {
            Err("username cannot be empty.".to_string())
        } else if is_too_long {
            Err("username cannot be longer than 256 characters.".to_string())
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for UserUsername {
    /// Performs the conversion.
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::UserUsername;
    use claims::{assert_err, assert_ok};

    // Must return `Err` if the input is using only whitespace.
    #[test]
    fn user_username_returns_err_when_filled_with_whitespace() {
        let name = " ".to_string();
        assert_err!(UserUsername::parse(name));
    }

    // Must return `Err` if the input is empty.
    #[test]
    fn user_username_returns_err_when_empty() {
        let name = "".to_string();
        assert_err!(UserUsername::parse(name));
    }

    // Must return `Ok` if the input is not bigger than 256 characters.
    #[test]
    fn user_username_returns_ok_when_not_too_long() {
        let name = "a".repeat(256);
        assert_ok!(UserUsername::parse(name));
    }

    // Must return `Err` if the input is bigger than 256 characters.
    #[test]
    fn user_username_returns_err_when_too_long() {
        let name = "a".repeat(257);
        assert_err!(UserUsername::parse(name));
    }

    // Must return `Ok` if the input is valid.
    #[test]
    fn user_username_returns_ok_when_valid() {
        let name = "username".to_string();
        assert_ok!(UserUsername::parse(name));
    }
}
