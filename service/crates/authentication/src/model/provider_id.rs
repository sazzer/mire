use mire_users::{AuthenticationProvider, AuthenticationProviderParseError};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;

/// The identity of an authentication provider
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Serialize, Deserialize)]
pub struct ProviderId(pub(crate) String);

/// Errors that can occur when parsing a Provider ID
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ProviderIdParseError {
    /// The Provider ID was blank
    #[error("The Provider ID was blank")]
    Blank,
}

impl FromStr for ProviderId {
    type Err = ProviderIdParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err(ProviderIdParseError::Blank)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

impl TryFrom<&ProviderId> for AuthenticationProvider {
    type Error = AuthenticationProviderParseError;

    fn try_from(provider_id: &ProviderId) -> Result<Self, Self::Error> {
        Self::from_str(&provider_id.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn parse_blank_provider_id() {
        let err = ProviderId::from_str("").unwrap_err();
        check!(err == ProviderIdParseError::Blank);
    }

    #[test]
    fn parse_whitespace_provider_id() {
        let err = ProviderId::from_str("   ").unwrap_err();
        check!(err == ProviderIdParseError::Blank);
    }

    #[test]
    fn parse_valid_provider_id() {
        let provider = ProviderId::from_str("google").unwrap();
        check!(provider.0 == "google");
    }

    #[test]
    fn parse_padded_provider_id() {
        let provider = ProviderId::from_str("  google  ").unwrap();
        check!(provider.0 == "google");
    }
}
