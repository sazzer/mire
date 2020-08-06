use serde::{Deserialize, Serialize};

/// The unique identifier for an authentication provider
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationProvider(String);

impl<S> From<S> for AuthenticationProvider
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

/// The unique identifier for a user at an authentication provider
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationId(String);

impl<S> From<S> for AuthenticationId
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
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
