use crate::{UserId, UserModel, UsersService};

impl UsersService {
    /// Get the single user that has the provided unique ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to retrieve
    ///
    /// # Returns
    /// The User that has the provided ID.
    /// If no user was found then instead returns `None`.
    #[tracing::instrument(skip(self))]
    pub async fn get_by_id(&self, user_id: &UserId) -> Option<UserModel> {
        self.repository.get_by_id(user_id).await
    }
}
