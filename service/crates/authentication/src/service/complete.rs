use super::AuthenticationService;
use crate::ProviderId;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug, thiserror::Error)]
pub enum CompleteAuthenticationError {
    #[error("The requested provider is not known")]
    UnknownProvider,

    #[error("Failed to authenticate the user")]
    AuthenticationFailure,
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

        let authenticated_user = provider.complete(params).await.ok_or_else(|| {
            tracing::warn!(provider_id = ?provider_id, "Failed to authenticate");
            CompleteAuthenticationError::AuthenticationFailure
        })?;

        let provider_id = provider_id.try_into().unwrap();
        let user = self
            .users_service
            .get_by_authentication(&provider_id, &authenticated_user.provider_id)
            .await;

        let _ = if let Some(authenticated_user) = user {
            tracing::debug!(user = ?authenticated_user, "Authenticated as user");
            authenticated_user
        } else {
            let user_data = mire_users::UserData {
                email: authenticated_user.email,
                display_name: authenticated_user.user_display_name,
                authentications: vec![mire_users::Authentication {
                    authentication_provider: provider_id,
                    authentication_id: authenticated_user.provider_id,
                    display_name: authenticated_user.provider_display_name,
                }],
            };
            let created_user = self.users_service.create(user_data).await.unwrap();
            tracing::debug!(user = ?created_user, "Registered as user");

            created_user
        };

        Ok(())
    }
}
