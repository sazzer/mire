use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// The unique identifier for an authentication provider
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationProvider(String);

/// Errors that can occur when parsing an Authentication Provider
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthenticationProviderParseError {
    /// The Authentication Provider was blank
    #[error("The Authentication Provider was blank")]
    Blank,
}

impl FromStr for AuthenticationProvider {
    type Err = AuthenticationProviderParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            Err(AuthenticationProviderParseError::Blank)
        } else {
            Ok(Self(input.to_owned()))
        }
    }
}

/// The unique identifier for a user at an authentication provider
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationId(String);

/// Errors that can occur when parsing an Authentication Id
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthenticationIdParseError {
    /// The Authentication Id was blank
    #[error("The Authentication Id was blank")]
    Blank,
}

impl FromStr for AuthenticationId {
    type Err = AuthenticationIdParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            Err(AuthenticationIdParseError::Blank)
        } else {
            Ok(Self(input.to_owned()))
        }
    }
}

/// The authentication details for a user from a single provider
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Authentication {
    /// The identity of the provider these details relate to
    pub authentication_provider: AuthenticationProvider,
    /// The ID of the user at this provider
    pub authentication_id: AuthenticationId,
    /// The display name of these details at the provider
    pub display_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn parse_blank_authentication_provider() {
        let err = AuthenticationProvider::from_str("").unwrap_err();
        check!(err == AuthenticationProviderParseError::Blank);
    }

    #[test]
    fn parse_valid_authentication_provider() {
        let provider = AuthenticationProvider::from_str("test@example.com").unwrap();

        check!(provider.0 == "test@example.com");
    }

    #[test]
    fn parse_blank_authentication_id() {
        let err = AuthenticationId::from_str("").unwrap_err();
        check!(err == AuthenticationIdParseError::Blank);
    }

    #[test]
    fn parse_valid_authentication_id() {
        let id = AuthenticationId::from_str("test@example.com").unwrap();

        check!(id.0 == "test@example.com");
    }
}
