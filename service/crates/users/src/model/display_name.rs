use bytes::BytesMut;
use postgres_types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// The display name for a User record
#[derive(Debug, PartialEq, FromSql, Serialize, Deserialize, Clone)]
pub struct DisplayName(String);

/// Errors that can occur when parsing a display name
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DisplayNameParseError {
    /// The display name was blank
    #[error("The display name was blank")]
    Blank,
}

impl FromStr for DisplayName {
    type Err = DisplayNameParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(DisplayNameParseError::Blank)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

impl ToSql for DisplayName {
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
        let err = DisplayName::from_str("").unwrap_err();
        check!(err == DisplayNameParseError::Blank);
    }

    #[test]
    fn parse_whitespace() {
        let err = DisplayName::from_str("   ").unwrap_err();
        check!(err == DisplayNameParseError::Blank);
    }

    #[test]
    fn parse_valid() {
        let display_name = DisplayName::from_str("Test User").unwrap();

        check!(display_name.0 == "Test User");
    }

    #[test]
    fn parse_padded() {
        let display_name = DisplayName::from_str("   Test User   ").unwrap();

        check!(display_name.0 == "Test User");
    }
}
