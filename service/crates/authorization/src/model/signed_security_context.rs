use biscuit::Compact;
use serde::{Deserialize, Serialize};

/// Representation of a security context that has been signed and is ready to send to a client.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SignedSecurityContext(String);

impl From<Compact> for SignedSecurityContext {
    fn from(jwt: Compact) -> Self {
        Self(jwt.to_string())
    }
}
