use secrecy::{ExposeSecret, Secret};
use unicode_segmentation::UnicodeSegmentation;

/// Representation of a user's password.
#[derive(Debug)]
pub struct UserPassword(Secret<String>);

impl UserPassword {
    /// Returns a valid password.
    pub fn parse(s: String) -> Result<Self, String> {
        // Check if the input is empty or with a trailing whitespace-like character.
        let is_empty_or_whitespace = s.trim().is_empty();
        // Check if the input is too short.
        let is_too_short = s.graphemes(true).count() < 8;
        // Check if the input is too long.
        let is_too_long = s.graphemes(true).count() > 256;

        // Validate whether the input is a valid password or not.
        if is_empty_or_whitespace {
            Err("password cannot be empty.".to_string())
        } else if is_too_short {
            Err("password must be at least 8 characters long.".to_string())
        } else if is_too_long {
            Err("password cannot be longer than 256 characters.".to_string())
        } else {
            Ok(Self(s.into()))
        }
    }
}

impl AsRef<str> for UserPassword {
    /// Performs the conversion.
    fn as_ref(&self) -> &str {
        self.0.expose_secret()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::UserPassword;
    use claims::{assert_err, assert_ok};

    // Must return `Err` if the input is using only whitespace.
    #[test]
    fn user_password_returns_err_when_filled_with_whitespace() {
        let name = " ".to_string();
        assert_err!(UserPassword::parse(name));
    }

    // Must return `Err` if the input is empty.
    #[test]
    fn user_password_returns_err_when_empty() {
        let name = "".to_string();
        assert_err!(UserPassword::parse(name));
    }

    // Must return `Ok` if the input is not smaller than 8 characters.
    #[test]
    fn user_password_returns_ok_when_not_too_short() {
        let name = "a".repeat(8);
        assert_ok!(UserPassword::parse(name));
    }

    // Must return `Err` if the input is smaller than 8 characters.
    #[test]
    fn user_password_returns_err_when_too_short() {
        let name = "a".repeat(7);
        assert_err!(UserPassword::parse(name));
    }

    // Must return `Ok` if the input is not bigger than 256 characters.
    #[test]
    fn user_password_returns_ok_when_not_too_long() {
        let name = "a".repeat(256);
        assert_ok!(UserPassword::parse(name));
    }

    // Must return `Err` if the input is bigger than 256 characters.
    #[test]
    fn user_password_returns_err_when_too_long() {
        let name = "a".repeat(257);
        assert_err!(UserPassword::parse(name));
    }

    // Must return `Ok` if the input is valid.
    #[test]
    fn user_password_returns_ok_when_valid() {
        let name = "fake_password".to_string();
        assert_ok!(UserPassword::parse(name));
    }
}
