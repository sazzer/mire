use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use std::str::FromStr;

/// The email address for a User record
#[derive(Debug, PartialEq, FromSql)]
pub struct Email(String);

/// Errors that can occur when parsing an Email Address
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EmailParseError {
    /// The Email was blank
    #[error("The Email was blank")]
    Blank,
}

impl FromStr for Email {
    type Err = EmailParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(EmailParseError::Blank)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

impl ToSql for Email {
    accepts!(TEXT, VARCHAR);

    to_sql_checked!();

    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn parse_blank() {
        let err = Email::from_str("").unwrap_err();
        check!(err == EmailParseError::Blank);
    }

    #[test]
    fn parse_whitespace() {
        let err = Email::from_str("   ").unwrap_err();
        check!(err == EmailParseError::Blank);
    }

    #[test]
    fn parse_valid() {
        let email = Email::from_str("test@example.com").unwrap();

        check!(email.0 == "test@example.com");
    }

    #[test]
    fn parse_padded() {
        let email = Email::from_str("   test@example.com   ").unwrap();

        check!(email.0 == "test@example.com");
    }
}
