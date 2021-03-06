use super::VerifyError;
use crate::{PrincipalId, SecurityContext, SecurityContextId};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Representation of the claims within the JWT that represents a signed security context.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityContextClaims {
    /// The ID of the JWT
    pub(super) jti: String,
    /// The Audience of the JWT
    pub(super) aud: String,
    /// The Issuer of the JWT
    pub(super) iss: String,
    /// The Subject of the JWT - literally the User ID
    pub(super) sub: Option<String>,
    /// When the JWT was issued
    pub(super) iat: i64,
    /// The timestamp before which the JWT is not valid
    pub(super) nbf: i64,
    /// When the JWT expires
    pub(super) exp: i64,
}

impl Default for SecurityContextClaims {
    fn default() -> Self {
        Self {
            jti: "".to_owned(),
            aud: "tag:mire,2020:authorization".to_owned(),
            iss: "tag:mire,2020:authorization".to_owned(),
            sub: None,
            iat: 0,
            nbf: 0,
            exp: 0,
        }
    }
}

impl From<&SecurityContext> for SecurityContextClaims {
    /// Convert a Security Context into a set of claims that are ready to be signed
    ///
    /// # Parameters
    /// - `security_context` - The security context to convert
    ///
    /// # Returns
    /// The set of claims
    fn from(security_context: &SecurityContext) -> Self {
        Self {
            jti: security_context.id.0.clone(),
            sub: match &security_context.principal_id {
                PrincipalId::User(user_id) => Some(user_id.clone()),
            },
            iat: security_context.not_valid_before.timestamp(),
            nbf: security_context.not_valid_before.timestamp(),
            exp: security_context.not_valid_after.timestamp(),
            ..Self::default()
        }
    }
}

impl TryFrom<SecurityContextClaims> for SecurityContext {
    type Error = VerifyError;

    /// Convert a set of claims representing a security context back into the security context
    ///
    /// # Parameters
    /// - `claims` - The claims to convert
    ///
    /// # Returns
    /// The security context
    fn try_from(claims: SecurityContextClaims) -> Result<Self, Self::Error> {
        Ok(Self {
            id: SecurityContextId(claims.jti),
            principal_id: claims
                .sub
                .map(PrincipalId::User)
                .ok_or(VerifyError::MissingPrincipal)?,
            not_valid_before: DateTime::from_utc(NaiveDateTime::from_timestamp(claims.nbf, 0), Utc),
            not_valid_after: DateTime::from_utc(NaiveDateTime::from_timestamp(claims.exp, 0), Utc),
        })
    }
}
