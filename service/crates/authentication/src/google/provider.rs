use super::Config;
use crate::service::Provider;

/// Implementation of the Provider trait for authenticating with Google
pub struct GoogleProvider {}

impl GoogleProvider {
    /// Construct the Google Authentication Provider
    ///
    /// # Parameters
    /// - `config` - The configuration to use for Google
    pub const fn new(_config: &Config) -> Self {
        Self {}
    }
}

impl Provider for GoogleProvider {}
