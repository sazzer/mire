use crate::{repository::SaveUserError, UserId, UserModel, UsersService};

/// Errors that can occur from updating a user record.
#[derive(Debug, thiserror::Error)]
pub enum UpdateUserError {
    #[error("The requested user was unknown")]
    UnknownUser,
    #[error("The version was not correct for the user record")]
    IncorrectVersion,
    #[error("The email address is registered to another user")]
    DuplicateEmail,
    #[error("An unexpected error occurred")]
    UnexpectedError,
}

impl UsersService {
    /// Update the user that has the provided unique ID by applying the provided lambda to it.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to retrieve
    /// - `updater` - The lambda to use to update the user details
    ///
    /// # Returns
    /// The newly modified user model.
    #[tracing::instrument(skip(self, updater))]
    pub async fn update<F>(
        &self,
        user_id: &UserId,
        updater: F,
    ) -> Result<UserModel, UpdateUserError>
    where
        F: Fn(&mut UserModel),
    {
        let mut user = self
            .repository
            .get_by_id(user_id)
            .await
            .ok_or(UpdateUserError::UnknownUser)?;

        updater(&mut user);

        let user = self.repository.update(user).await?;
        Ok(user)
    }
}

impl From<SaveUserError> for UpdateUserError {
    fn from(e: SaveUserError) -> Self {
        match e {
            SaveUserError::DuplicateEmail => Self::DuplicateEmail,
            SaveUserError::UnknownUser => Self::UnknownUser,
            SaveUserError::IncorrectVersion => Self::IncorrectVersion,
            SaveUserError::UnexpectedError => Self::UnexpectedError,
        }
    }
}
