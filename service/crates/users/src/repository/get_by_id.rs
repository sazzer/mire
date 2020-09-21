use super::UsersRepository;
use crate::{UserId, UserModel};

impl UsersRepository {
    /// Get the single user that has the provided unique ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to retrieve
    ///
    /// # Returns
    /// The User that has the provided ID.
    /// If no user was found then instead returns `None`.
    pub async fn get_by_id(&self, user_id: &UserId) -> Option<UserModel> {
        tracing::debug!(user_id = ?user_id, "Loading user by ID");

        let conn = self.database.checkout().await.unwrap();
        let user = conn
            .query_opt("SELECT * FROM users WHERE user_id = $1", &[&user_id])
            .await
            .unwrap()
            .map(UserModel::from);

        tracing::debug!(user = ?user, "Loaded user by ID");
        user
    }
}
