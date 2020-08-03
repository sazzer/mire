use super::claims::SecurityContextClaims;
use crate::{AuthorizationService, SecurityContext, SignedSecurityContext};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PrincipalId, SigningKey};
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
}
