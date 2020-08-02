use crate::{
    AuthorizationService, PrincipalId, SecurityContext, SecurityContextId, SignedSecurityContext,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

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
            aud: "tag:mire,2020:authorization".to_owned(),
            iss: "tag:mire,2020:authorization".to_owned(),
            sub: match &security_context.principal_id {
                PrincipalId::User(user_id) => Some(user_id.clone()),
            },
            iat: security_context.not_valid_before.timestamp(),
            nbf: security_context.not_valid_before.timestamp(),
            exp: security_context.not_valid_after.timestamp(),
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<SecurityContextClaims> for SecurityContext {
    /// Convert a set of claims representing a security context back into the security context
    ///
    /// # Parameters
    /// - `claims` - The claims to convert
    ///
    /// # Returns
    /// The security context
    ///
    /// # Todos
    /// - TODO: Convert this to `TryFrom` so that it can fail.
    fn from(claims: SecurityContextClaims) -> Self {
        Self {
            id: SecurityContextId(claims.jti),
            principal_id: claims.sub.map(PrincipalId::User).unwrap(),
            not_valid_before: DateTime::from_utc(NaiveDateTime::from_timestamp(claims.nbf, 0), Utc),
            not_valid_after: DateTime::from_utc(NaiveDateTime::from_timestamp(claims.exp, 0), Utc),
        }
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
    #[must_use]
    pub fn verify(&self, security_context: &SignedSecurityContext) -> SecurityContext {
        let token = decode::<SecurityContextClaims>(
            &security_context.0,
            &DecodingKey::from_secret(self.signing_key.0.as_ref()),
            &Validation {
                algorithms: vec![Algorithm::HS512],
                ..Validation::default()
            },
        )
        .unwrap();

        token.claims.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PrincipalId, SigningKey};
    use assert2::check;
    use chrono::Duration;

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
    fn test_sign_verify() {
        let service = AuthorizationService {
            security_context_duration: Duration::days(5),
            signing_key: SigningKey::new("test"),
        };
        let principal = PrincipalId::User("userId".to_owned());

        let security_context = service.generate_security_context(principal);

        let signed = service.sign(&security_context);

        let verified = service.verify(&signed);
        check!(verified == security_context);
    }
}
