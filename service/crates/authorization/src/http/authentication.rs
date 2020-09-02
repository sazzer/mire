use super::Authenticator;
use crate::{PrincipalId, SecurityContext};
use actix_http::http::StatusCode;
use mire_problem::{Problem, SimpleProblemType};

/// Problem type to indicate that the request has invalid authentication details.
pub const FORBIDDEN: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:mire/2020:problems/forbidden",
    problem_title: "The request was not permitted to perform this action",
    status_code: StatusCode::FORBIDDEN,
};

/// Enumeration of the possible statuses of authentication
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AuthenticationStatus {
    /// Authentication has passed.
    Passed,
    /// Authentication has failed.
    Failed,
}

impl From<AuthenticationStatus> for Result<(), Problem> {
    fn from(status: AuthenticationStatus) -> Self {
        if status == AuthenticationStatus::Passed {
            Ok(())
        } else {
            Err(Problem::new(FORBIDDEN))
        }
    }
}

/// Struct to represent an active authentication check
#[derive(Debug)]
pub struct Authentication<'a> {
    /// The Security Context that is being authenticated. May be None.
    security_context: &'a Option<SecurityContext>,
    /// The current result of authentication.
    result: AuthenticationStatus,
}

impl Authenticator {
    /// Start checking the authentication details of this request.
    ///
    /// # Returns
    /// The `Authentication` object to perform the checks with.
    #[must_use]
    pub const fn check(&self) -> Authentication {
        Authentication {
            security_context: &self.0,
            result: AuthenticationStatus::Passed,
        }
    }
}

impl<'a> Authentication<'a> {
    /// Produce a new Authentication object representing the same Security Context but with a new Pass result.
    ///
    /// If the current status is `AuthenticationStatus::Passed` then the new one is as well.
    /// If the current status is `AuthenticationStatus::Failed` then the new one is as well.
    ///
    /// # Returns
    /// The new Authentication details.
    const fn pass(self) -> Self {
        self
    }

    /// Produce a new Authentication object representing the same Security Context but with a new Fail result.
    ///
    /// Regardless of the current status, the new status is `AuthenticationStatus::Failed`
    ///
    /// # Returns
    /// The new Authentication details.
    const fn fail(self) -> Self {
        Self {
            security_context: self.security_context,
            result: AuthenticationStatus::Failed,
        }
    }

    /// Authenticate if we actually have a Security Context. If we do then this check will pass. If we don't then
    /// this check will fail.
    ///
    /// # Returns
    /// The new Authentication details.
    pub fn authenticated(self) -> Self {
        if self.security_context.is_some() {
            self.pass()
        } else {
            self.fail()
        }
    }

    /// Authenticate if we have a Security Context representing the same Principal as the one requested.
    /// If we don't have a security context, or it represents a different Principal then this check will fail.
    ///
    /// # Parameters
    /// - `principal` - The Principal to compare against
    ///
    /// # Returns
    /// The new Authentication details.
    pub fn same_principal(self, principal: &PrincipalId) -> Self {
        if self.security_context.as_ref().map(|sc| &sc.principal_id) == Some(principal) {
            self.pass()
        } else {
            self.fail()
        }
    }

    /// Get the status of the authentication.
    ///
    /// # Returns
    /// The status of the authentication.
    pub const fn status(&self) -> AuthenticationStatus {
        self.result
    }

    /// Get the status of the authentication as a Result.
    ///
    /// # Returns
    /// The status of the authentication.
    /// If the status was `Passed` then this returns a successful `Result` with no content.
    /// If the status was `Failed` then this returns an error `Result` containing a `Problem` to send to the client.
    pub fn result(&self) -> Result<(), Problem> {
        self.result.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SecurityContextId;
    use assert2::check;
    use chrono::Utc;

    #[test]
    fn test_no_checks() {
        let authenticator = Authenticator(None);
        let status = authenticator.check();
        check!(status.status() == AuthenticationStatus::Passed);
        check!(
            status
                .result()
                .map_err(|problem| problem.error.problem_type())
                == Ok(())
        );
    }

    #[test]
    fn test_not_authenticated() {
        let authenticator = Authenticator(None);
        let status = authenticator.check().authenticated();
        check!(status.status() == AuthenticationStatus::Failed);
        check!(
            status
                .result()
                .map_err(|problem| problem.error.problem_type())
                == Err("tag:mire/2020:problems/forbidden")
        );
    }

    #[test]
    fn test_authenticated() {
        let security_context = SecurityContext {
            id: SecurityContextId::default(),
            not_valid_before: Utc::now(),
            not_valid_after: Utc::now(),
            principal_id: PrincipalId::User("user".to_owned()),
        };

        let authenticator = Authenticator(Some(security_context));
        let status = authenticator.check().authenticated();
        check!(status.status() == AuthenticationStatus::Passed);
        check!(
            status
                .result()
                .map_err(|problem| problem.error.problem_type())
                == Ok(())
        );
    }

    #[test]
    fn test_no_principal() {
        let authenticator = Authenticator(None);
        let status = authenticator
            .check()
            .same_principal(&PrincipalId::User("user".to_owned()));
        check!(status.status() == AuthenticationStatus::Failed);
        check!(
            status
                .result()
                .map_err(|problem| problem.error.problem_type())
                == Err("tag:mire/2020:problems/forbidden")
        );
    }

    #[test]
    fn test_wrong_principal() {
        let security_context = SecurityContext {
            id: SecurityContextId::default(),
            not_valid_before: Utc::now(),
            not_valid_after: Utc::now(),
            principal_id: PrincipalId::User("user".to_owned()),
        };

        let authenticator = Authenticator(Some(security_context));
        let status = authenticator
            .check()
            .same_principal(&PrincipalId::User("other".to_owned()));
        check!(status.status() == AuthenticationStatus::Failed);
        check!(
            status
                .result()
                .map_err(|problem| problem.error.problem_type())
                == Err("tag:mire/2020:problems/forbidden")
        );
    }

    #[test]
    fn test_right_principal() {
        let security_context = SecurityContext {
            id: SecurityContextId::default(),
            not_valid_before: Utc::now(),
            not_valid_after: Utc::now(),
            principal_id: PrincipalId::User("user".to_owned()),
        };

        let authenticator = Authenticator(Some(security_context));
        let status = authenticator
            .check()
            .same_principal(&PrincipalId::User("user".to_owned()));
        check!(status.status() == AuthenticationStatus::Passed);
        check!(
            status
                .result()
                .map_err(|problem| problem.error.problem_type())
                == Ok(())
        );
    }
}
