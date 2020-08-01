mod generate;
mod sign;
mod signing_key;

use biscuit::jws::Secret;
use chrono::Duration;
pub use signing_key::SigningKey;

/// The service layer for managing authorization of principals
pub struct AuthorizationService {
    /// The validity duration of generated security contexts
    #[allow(dead_code)]
    pub(super) security_context_duration: Duration,

    /// The key with which to sign security contexts
    #[allow(dead_code)]
    pub(super) secret: Secret,
}
