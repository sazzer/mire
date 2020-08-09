use super::Config;
use crate::{service::Provider, StartAuthentication};

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

impl Provider for GoogleProvider {
    /// Generate the details needed to redirect the user to authenticate with this provider
    fn start(&self) -> StartAuthentication {
        todo!()
    }
}
