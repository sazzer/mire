use crate::{AuthorizationService, PrincipalId, SecurityContext, SignedSecurityContext};
use biscuit::jwa::SignatureAlgorithm;
use biscuit::jws::RegisteredHeader;
use biscuit::{ClaimsSet, RegisteredClaims, SingleOrMultiple, JWT};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct SecurityContextClaims {}

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
        let subject = match &security_context.principal_id {
            PrincipalId::User(user_id) => Some(FromStr::from_str(user_id).unwrap()),
        };

        let claims = ClaimsSet::<SecurityContextClaims> {
            registered: RegisteredClaims {
                id: Some(security_context.id.0.to_string()),
                issued_at: Some(security_context.not_valid_before.into()),
                not_before: Some(security_context.not_valid_before.into()),
                expiry: Some(security_context.not_valid_after.into()),
                audience: Some(SingleOrMultiple::Single(
                    FromStr::from_str("tag:mire,2020:authorization").unwrap(),
                )),
                issuer: Some(FromStr::from_str("tag:mire,2020:authorization").unwrap()),
                subject,
            },
            private: SecurityContextClaims {},
        };
        let header = RegisteredHeader {
            algorithm: SignatureAlgorithm::HS512,
            ..RegisteredHeader::default()
        };
        let jwt = JWT::new_decoded(header.into(), claims);
        let signed = jwt.into_encoded(&self.secret).unwrap().unwrap_encoded();

        tracing::debug!(security_context = ?security_context, signed = ?signed, "Generated signed security context");
        signed.into()
    }

    // /// Verify the given signed security context and return the expanded version
    // ///
    // /// # Parameters
    // /// - `security_context` - The security context to verify
    // ///
    // /// # Returns
    // /// The expanded security context
    // #[must_use]
    // pub fn verify(&self, _security_context: &SignedSecurityContext) -> SecurityContext {
    //     todo!()
    // }
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
            secret: SigningKey::new("test").into(),
        };
        let principal = PrincipalId::User("userId".to_owned());

        let security_context = service.generate_security_context(principal);

        let _ = service.sign(&security_context);
    }
}
