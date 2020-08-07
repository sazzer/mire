use crate::{AuthenticationId, AuthenticationProvider, UserData, UserModel, UsersService};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CreateUserError {
    /// The email address is already registered
    #[error("The email address is already registered")]
    DuplicateEmail,

    /// The authentication details are already registered
    #[error("The authentication details are already registered")]
    DuplicateAuthentication(AuthenticationProvider, AuthenticationId),

    /// An unexpected error occurred
    #[error("An unexpected error occurred")]
    UnexpectedError,
}

impl UsersService {
    /// Create a new user with the provided details to describe them.
    ///
    /// # Parameters
    /// - `data` - The details of the user to create
    ///
    /// # Returns
    /// The newly created user
    ///
    /// # Errors
    /// If the user is unable to be created for any reason
    #[tracing::instrument(skip(self))]
    pub async fn create(&self, data: UserData) -> Result<UserModel, CreateUserError> {
        for authentication in &data.authentications {
            if self
                .get_by_authentication(
                    &authentication.authentication_provider,
                    &authentication.authentication_id,
                )
                .await
                .is_some()
            {
                tracing::warn!(authentication = ?authentication, "Attempting to create user with duplicate authentication details");
                return Err(CreateUserError::DuplicateAuthentication(
                    authentication.authentication_provider.clone(),
                    authentication.authentication_id.clone(),
                ));
            }
        }

        todo!()
    }
}
