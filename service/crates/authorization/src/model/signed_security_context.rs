use serde::{Deserialize, Serialize};

/// Representation of a security context that has been signed and is ready to send to a client.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SignedSecurityContext(pub(crate) String);

impl AsRef<str> for SignedSecurityContext {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
