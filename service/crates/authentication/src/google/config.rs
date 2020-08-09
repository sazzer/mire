use super::provider::GoogleProvider;
use crate::config::AuthenticationConfig;
use std::sync::Arc;

/// Configuration needed to authenticate with Google
#[derive(Debug)]
pub struct Config {
    /// The Client ID
    pub client_id: String,
    /// The Client Secret
    pub client_secret: String,
    /// The URI for Google to redirect to after authentication
    pub redirect_uri: String,

    /// The URI to redirect to in order to start authentication
    pub auth_uri: Option<String>,
    /// The URI to call to get the authenticated token
    pub token_uri: Option<String>,
}

impl AuthenticationConfig {
    /// Add Google Authentication to the authentication component
    ///
    /// # Parameters
    /// - `config` - The Google configuration
    pub fn with_google(&mut self, config: &Config) {
        tracing::info!(config = ?config, "Registering Google Authentication");

        self.registry.insert(
            "google".parse().unwrap(),
            Arc::new(GoogleProvider::new(config)),
        );
    }
}
