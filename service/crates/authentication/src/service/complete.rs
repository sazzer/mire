use super::AuthenticationService;
use crate::ProviderId;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum CompleteAuthenticationError {
    #[error("The requested provider is not known")]
    UnknownProvider,
}

impl AuthenticationService {
    /// Complete authenticating against the requested provider
    ///
    /// # Parameters
    /// - `provider_id` - The ID of the provider to authenticate against
    /// - `params` - The parameters from the callback with which to complete authentication
    ///
    /// # Returns
    /// The details needed to complete authentiaction
    ///
    /// # Errors
    /// If anything goes wrong completeing authentication
    pub async fn complete_authentication(
        &self,
        provider_id: &ProviderId,
        params: &HashMap<String, String>,
    ) -> Result<(), CompleteAuthenticationError> {
        let provider = self.registry.get_provider(provider_id).ok_or_else(|| {
            tracing::warn!(provider_id = ?provider_id, "Unknown provider requested");
            CompleteAuthenticationError::UnknownProvider
        })?;

        provider.complete(params).await;

        Ok(())
    }
}
