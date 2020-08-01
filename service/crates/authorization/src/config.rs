use super::{AuthorizationService, SigningKey};
use chrono::Duration;

/// Configuration of the authorization component.
pub struct AuthorizationConfig {
    /// The authorization service
    pub service: AuthorizationService,
}

impl AuthorizationConfig {
    /// Construct the authorization component.
    #[must_use]
    pub const fn new(security_context_duration: Duration, signing_key: SigningKey) -> Self {
        let service = AuthorizationService {
            security_context_duration,
            signing_key,
        };
        Self { service }
    }
}
