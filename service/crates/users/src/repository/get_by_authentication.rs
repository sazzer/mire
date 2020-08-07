use super::UsersRepository;
use crate::{AuthenticationId, AuthenticationProvider, UserModel};
use serde_json::json;

impl UsersRepository {
    /// Get the single user that has the provided unique ID at the provided Authentication Provider.
    ///
    /// # Parameters
    /// - `provider` - The Provider to whom the ID belongs.
    /// - `provider_id` - The ID of the user at the given provider.
    ///
    /// # Returns
    /// The User that has the provided ID at the given provider.
    /// If no user was found then instead returns `None`.
    pub async fn get_by_authentication(
        &self,
        provider: &AuthenticationProvider,
        provider_id: &AuthenticationId,
    ) -> Option<UserModel> {
        tracing::debug!("Loading user by authentication details");

        let authentication_bind = json!([{
          "authentication_provider": provider,
          "authentication_id": provider_id
        }]);

        let conn = self.database.checkout().await.unwrap();
        let user = conn
            .query_opt(
                "SELECT * FROM users WHERE authentications @> $1",
                &[&authentication_bind],
            )
            .await
            .unwrap()
            .map(UserModel::from);

        tracing::debug!(user = ?user, "Loaded user by authentication details");
        user
    }
}
