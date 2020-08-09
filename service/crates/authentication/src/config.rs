use crate::service::Registry;
use std::sync::Arc;

/// Configuration of the authentication component.
pub struct AuthenticationConfig {
    /// The registry of authentication providers
    pub registry: Registry,
}

impl AuthenticationConfig {
    /// Construct the authentication component.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let registry = Registry::default();
        Self { registry }
    }

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    #[must_use]
    pub fn server_config(&self) -> mire_server::FnConfig {
        let registry = self.registry.clone();

        Arc::new(move |c| {
            c.data(registry.clone());

            c.service(super::endpoints::list_providers::list_providers);
        })
    }
}
