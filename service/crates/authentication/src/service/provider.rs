use async_trait::async_trait;

/// Trait that represents an Authentication Provider of some kind.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Generate the details needed to redirect the user to authenticate with this provider
    ///
    /// # Parameters
    /// - `nonce` - A generated nonce unique to this request
    ///
    /// # Returns
    /// The URL to redirect the user to in order to start authentication
    fn start(&self, nonce: &str) -> String;

    /// Complete authentication against the provider and return details of the user that authenticated
    ///
    /// # Parameters
    /// - `params` - The parameters received from the callback from the provider
    ///
    /// # Returns
    /// The details of the user that authenticated
    ///
    /// # Errors
    /// If an error occurs authenticating against the provider
    async fn complete(&self, params: &std::collections::HashMap<String, String>);
}

#[cfg(test)]
pub struct ProviderMock {}

#[cfg(test)]
impl ProviderMock {
    pub const fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
#[async_trait]
impl Provider for ProviderMock {
    fn start(&self, _nonce: &str) -> String {
        todo!()
    }

    async fn complete(&self, params: &std::collections::HashMap<String, String>) {
        todo!()
    }
}
