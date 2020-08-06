use crate::{AuthenticationId, AuthenticationProvider, UserModel, UsersService};

impl UsersService {
    /// Get the single user that has the provided unique ID at the provided Authentication Provider.
    ///
    /// # Parameters
    /// - `provider` - The Provider to whom the ID belongs.
    /// - `provider_id` - The ID of the user at the given provider.
    ///
    /// # Returns
    /// The User that has the provided ID at the given provider.
    /// If no user was found then instead returns `None`.
    #[tracing::instrument(skip(self))]
    pub async fn get_by_authentication(
        &self,
        provider: &AuthenticationProvider,
        provider_id: &AuthenticationId,
    ) -> Option<UserModel> {
        None
    }
}
