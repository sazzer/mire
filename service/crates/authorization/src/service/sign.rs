// TODO: Split across files
use crate::{
    AuthorizationService, PrincipalId, SecurityContext, SecurityContextId, SignedSecurityContext,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum VerifyError {
    #[error("No principal was present in the security context")]
    MissingPrincipal,

    #[error("The security context has expired")]
    Expired,

    #[error("The security context was malformed in some way")]
    Malformed,

    #[error("An unexpected error occurred")]
    UnexpectedError,
}

/// Representation of the claims within the JWT that represents a signed security context.
#[derive(Debug, Serialize, Deserialize)]
struct SecurityContextClaims {
    /// The ID of the JWT
    jti: String,
    /// The Audience of the JWT
    aud: String,
    /// The Issuer of the JWT
    iss: String,
    /// The Subject of the JWT - literally the User ID
    sub: Option<String>,
    /// When the JWT was issued
    iat: i64,
    /// The timestamp before which the JWT is not valid
    nbf: i64,
    /// When the JWT expires
    exp: i64,
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

impl AuthorizationService {
    /// Sign the given security context and return the signed version ready for the client
    ///
    /// # Parameters
    /// - `security_context` - The security context to sign
    ///
    /// # Returns
    /// The signed security context
    #[must_use]
    pub fn sign(&self, security_context: &SecurityContext) -> SignedSecurityContext {
        let claims: SecurityContextClaims = security_context.into();
        let token = encode(
            &Header::new(Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(self.signing_key.0.as_ref()),
        )
        .unwrap();
        SignedSecurityContext(token)
    }

    /// Verify the given signed security context and return the expanded version
    ///
    /// # Parameters
    /// - `security_context` - The security context to verify
    ///
    /// # Returns
    /// The expanded security context
    ///
    /// # Errors
    /// If anything was invalid about the signed security context
    pub fn verify(
        &self,
        security_context: &SignedSecurityContext,
    ) -> Result<SecurityContext, VerifyError> {
        let mut valid_audiences = std::collections::HashSet::new();
        valid_audiences.insert("tag:mire,2020:authorization".to_owned());

        let token = decode::<SecurityContextClaims>(
            &security_context.0,
            &DecodingKey::from_secret(self.signing_key.0.as_ref()),
            &Validation {
                iss: Some("tag:mire,2020:authorization".to_owned()),
                aud: Some(valid_audiences),
                algorithms: vec![Algorithm::HS512],
                ..Validation::default()
            },
        )
        .map_err(|err| {
            tracing::warn!(err = ?err, security_context = ?security_context, "Error verifying security context");

            match err.into_kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => VerifyError::Expired,
                jsonwebtoken::errors::ErrorKind::InvalidSignature |
                jsonwebtoken::errors::ErrorKind::InvalidToken |
                jsonwebtoken::errors::ErrorKind::InvalidAlgorithm |
                jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName |
                jsonwebtoken::errors::ErrorKind::InvalidAudience | 
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => VerifyError::Malformed,
                _ => VerifyError::UnexpectedError,
            }
        })?;

        token.claims.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PrincipalId, SigningKey};
    use assert2::check;
    use chrono::Duration;

    const EARLY_TIMESTAMP: i64 = 1_496_437_337;
    const LATE_TIMESTAMP: i64 = 2_496_437_337;

    #[test]
    fn test_sign_valid() {
        let service = AuthorizationService {
            security_context_duration: Duration::days(5),
            signing_key: SigningKey::new("test"),
        };
        let principal = PrincipalId::User("userId".to_owned());

        let security_context = service.generate_security_context(principal);

        let _ = service.sign(&security_context);
    }

    #[test]
    fn test_verify_valid() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap();

        check!(verified.id == SecurityContextId("securityContextId".to_owned()));
        check!(verified.principal_id == PrincipalId::User("securityContextPrincipal".to_owned()));
        check!(verified.not_valid_before.timestamp() == EARLY_TIMESTAMP);
        check!(verified.not_valid_after.timestamp() == LATE_TIMESTAMP);
    }

    #[test]
    fn test_verify_no_principal() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: None,
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == VerifyError::MissingPrincipal);
    }

    #[test]
    fn test_verify_expired() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: EARLY_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == VerifyError::Expired);
    }

    #[test]
    fn test_verify_wrong_key() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test2",
        )
        .unwrap_err();

        check!(verified == VerifyError::Malformed);
    }

    #[test]
    fn test_verify_wrong_algorithm() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS256),
                &SecurityContextClaims {
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == VerifyError::Malformed);
    }

    #[test]
    fn test_verify_wrong_audience() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    aud: "incorrect".to_owned(),
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == VerifyError::Malformed);
    }

    #[test]
    fn test_verify_wrong_issuer() {
        let verified = help_verify_token(
            encode(
                &Header::new(Algorithm::HS512),
                &SecurityContextClaims {
                    iss: "incorrect".to_owned(),
                    jti: "securityContextId".to_owned(),
                    sub: Some("securityContextPrincipal".to_owned()),
                    iat: EARLY_TIMESTAMP,
                    nbf: EARLY_TIMESTAMP,
                    exp: LATE_TIMESTAMP,
                    ..SecurityContextClaims::default()
                },
                &EncodingKey::from_secret(b"test"),
            )
            .unwrap(),
            "test",
        )
        .unwrap_err();

        check!(verified == VerifyError::Malformed);
    }

    #[test]
    fn test_verify_malformed_token() {
        let verified = help_verify_token("malformed".to_owned(), "test2").unwrap_err();

        check!(verified == VerifyError::Malformed);
    }

    fn help_verify_token(token: String, secret: &str) -> Result<SecurityContext, VerifyError> {
        let service = AuthorizationService {
            security_context_duration: Duration::days(5),
            signing_key: SigningKey::new(secret),
        };

        let signed = SignedSecurityContext(token);

        service.verify(&signed)
    }
}
