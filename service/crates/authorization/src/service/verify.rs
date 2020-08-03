use crate::{
  AuthorizationService, SecurityContext, SignedSecurityContext,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::convert::TryInto;
use super::claims::SecurityContextClaims;
use super::VerifyError;

impl AuthorizationService {

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
  use crate::{PrincipalId, SigningKey, SecurityContextId};
  use assert2::check;
  use chrono::Duration;
  use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};

  const EARLY_TIMESTAMP: i64 = 1_496_437_337;
  const LATE_TIMESTAMP: i64 = 2_496_437_337;

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
