use crate::service::AuthenticatedUser;
use jsonwebtoken::dangerous_insecure_decode;
use mire_users::{AuthenticationId, DisplayName, Email};
use serde::Deserialize;
use std::convert::TryFrom;
use std::str::FromStr;

/// Representation of the ID Token received from Google
#[derive(Debug, Deserialize)]
pub struct IdToken(String);

/// Representation of an Access Token received from Google
#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub id_token: IdToken,
}

/// Expanded form of the JWT Claims from inside the ID Token
#[derive(Debug, Deserialize)]
pub struct IdTokenClaims {
    /// The subject of the ID Token. This is the User ID at Google
    sub: AuthenticationId,
    /// The email address of the subject.
    email: String,
    /// The display name of the subject.
    name: String,
}

impl TryFrom<IdToken> for IdTokenClaims {
    type Error = ();

    /// Parse the given `IdToken` string into the expanded claims
    ///
    /// # Parameters
    /// - `id_token` - The ID Token to convert
    ///
    /// # Returns
    /// The expanded claims
    fn try_from(id_token: IdToken) -> Result<Self, Self::Error> {
        dangerous_insecure_decode::<Self>(&id_token.0)
            .map(|jwt| jwt.claims)
            .map_err(|e| {
                tracing::warn!(e = ?e, id_token = ?id_token, "Failed to parse ID Token");
            })
    }
}

impl TryFrom<IdTokenClaims> for AuthenticatedUser {
    type Error = ();

    fn try_from(claims: IdTokenClaims) -> Result<Self, Self::Error> {
        let email = Email::from_str(&claims.email).map_err(|e| {
            tracing::warn!(e = ?e, email = ?claims.email, "Failed to parse email address");
        })?;

        let user_display_name = DisplayName::from_str(&claims.name).map_err(|e| {
            tracing::warn!(e = ?e, display_name = ?claims.name, "Failed to parse display name");
        })?;

        Ok(Self {
            provider_id: claims.sub,
            provider_display_name: claims.email,
            user_display_name,
            email,
        })
    }
}
