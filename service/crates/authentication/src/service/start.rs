use super::AuthenticationService;
use crate::{ProviderId, StartAuthentication};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum StartAuthenticationError {
    #[error("The requested provider is not known")]
    UnknownProvider,
}

impl AuthenticationService {
    /// Start authenticating against the requested provider
    ///
    /// # Parameters
    /// - `provider_id` - The ID of the provider to authenticate against
    ///
    /// # Returns
    /// The details needed to start authentiaction
    ///
    /// # Errors
    /// If anything goes wrong starting authentication
    pub fn start_authentication(
        &self,
        provider_id: &ProviderId,
    ) -> Result<StartAuthentication, StartAuthenticationError> {
        let provider = self.registry.get_provider(provider_id).ok_or_else(|| {
            tracing::warn!(provider_id = ?provider_id, "Unknown provider requested");
            StartAuthenticationError::UnknownProvider
        })?;

        let nonce = Uuid::new_v4().to_string();
        let redirect_uri = provider.start(&nonce);

        Ok(StartAuthentication {
            nonce,
            redirect_uri,
        })
    }
}
