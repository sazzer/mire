use super::TestSubject;
use mire_authorization::{PrincipalId, SignedSecurityContext};

impl TestSubject {
    /// Generate a valid access token for the provided User ID.
    /// Note that this does not ensure that the User ID is valid.
    ///
    /// # Parameters
    /// - `user_id` - The User ID to generate the access token for
    ///
    /// # Returns
    /// The access token.
    #[must_use]
    pub fn generate_access_token<S>(&self, user_id: S) -> SignedSecurityContext
    where
        S: Into<String>,
    {
        self.service
            .generate_access_token(PrincipalId::User(user_id.into()))
    }
}
