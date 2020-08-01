use crate::{AuthorizationService, PrincipalId, SecurityContext, SecurityContextId};
use chrono::Utc;

impl AuthorizationService {
    /// Generate a new security context for the provided Principal ID
    ///
    /// # Parameters
    /// - `principal` - The Principal ID to generate a Security Context for
    ///
    /// # Returns
    /// The Security Context
    #[must_use]
    pub fn generate_security_context(&self, principal: PrincipalId) -> SecurityContext {
        let now = Utc::now();
        let valid_until = now + self.security_context_duration;

        let id = SecurityContextId::default();

        SecurityContext {
            id,
            principal_id: principal,
            not_valid_before: now,
            not_valid_after: valid_until,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SigningKey;
    use assert2::check;
    use chrono::Duration;

    #[test]
    fn test_generate_security_context() {
        let service = AuthorizationService {
            security_context_duration: Duration::days(5),
            signing_key: SigningKey::new("test"),
        };

        let principal = PrincipalId::User("userId".to_owned());

        let security_context = service.generate_security_context(principal.clone());

        check!(principal == security_context.principal_id);
        check!(
            security_context.not_valid_before + Duration::days(5)
                == security_context.not_valid_after
        );
    }
}
