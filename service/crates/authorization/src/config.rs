use super::{AuthorizationService, SigningKey};
use chrono::Duration;
use std::sync::Arc;

/// Configuration of the authorization component.
pub struct AuthorizationConfig {
    /// The authorization service
    pub service: Arc<AuthorizationService>,
}

impl AuthorizationConfig {
    /// Construct the authorization component.
    #[must_use]
    pub fn new(security_context_duration: Duration, signing_key: SigningKey) -> Self {
        let service = AuthorizationService {
            security_context_duration,
            signing_key,
        };
        Self {
            service: Arc::new(service),
        }
    }

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    #[must_use]
    pub fn server_config(&self) -> mire_server::FnConfig {
        let service = self.service.clone();

        Arc::new(move |c| {
            c.data(service.clone());
        })
    }
}
