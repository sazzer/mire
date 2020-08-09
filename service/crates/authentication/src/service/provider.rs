use crate::StartAuthentication;
use async_trait::async_trait;

/// Trait that represents an Authentication Provider of some kind.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Generate the details needed to redirect the user to authenticate with this provider
    fn start(&self) -> StartAuthentication;
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
    fn start(&self) -> StartAuthentication {
        todo!()
    }
}
