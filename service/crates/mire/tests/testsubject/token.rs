use super::TestSubject;
use actix_web::{
    error::ParseError,
    http::header::{
        Header, HeaderName, HeaderValue, IntoHeaderValue, InvalidHeaderValue, AUTHORIZATION,
    },
};
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
    pub fn generate_access_token<S>(&self, user_id: S) -> impl Header
    where
        S: Into<String>,
    {
        let token = self
            .service
            .generate_access_token(PrincipalId::User(user_id.into()));

        Authorization(token)
    }
}

/// Representation of the Authorization header containing a signed security context
struct Authorization(SignedSecurityContext);

impl IntoHeaderValue for Authorization {
    type Error = InvalidHeaderValue;

    fn try_into(self) -> Result<HeaderValue, Self::Error> {
        let value = format!("Bearer {}", self.0);
        HeaderValue::from_str(&value)
    }
}
impl Header for Authorization {
    fn name() -> HeaderName {
        AUTHORIZATION
    }

    fn parse<T>(_: &T) -> Result<Self, ParseError> {
        todo!()
    }
}
