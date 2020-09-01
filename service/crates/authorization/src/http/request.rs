use super::Authenticator;
use crate::{AuthorizationService, SignedSecurityContext};
use actix_http::http::StatusCode;
use actix_web::{dev, http::header, web::Data, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use mire_problem::{Problem, SimpleProblemType};

pub const UNAUTHENTICATED: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:mire/2020:problems/unauthenticated",
    problem_title: "The request was not correctly authenticated",
    status_code: StatusCode::UNAUTHORIZED,
};

impl Authenticator {
    fn parse_header(
        header: Option<&header::HeaderValue>,
    ) -> Result<Option<SignedSecurityContext>, ()> {
        if let Some(authorization) = header {
            // Header value is unparsable
            let authorization = authorization.to_str().map_err(|e| {
                tracing::warn!(
                    "Failed to parse authorization header {:?}: {:?}",
                    authorization,
                    e
                );
            })?;

            if !authorization.starts_with("Bearer ") {
                tracing::warn!(
                    "Authorization header was not a Bearer token {:?}",
                    authorization
                );
                return Err(());
            }

            let authorization = &authorization[7..];
            tracing::debug!("Extracted signed security context: {:?}", authorization);
            Ok(Some(SignedSecurityContext(authorization.to_owned())))
        } else {
            // No header value
            Ok(None)
        }
    }
}

impl FromRequest for Authenticator {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let authorization = req.headers().get(header::AUTHORIZATION);
        tracing::debug!("Processing authorization header: {:?}", authorization);

        match Self::parse_header(authorization) {
            // No header = Authenticator with no security context
            Ok(None) => ok(Self(None)),
            // Valid header = Attempt to parse it and handle the response
            Ok(Some(signed_security_context)) => {
                let service: &Data<AuthorizationService> = req.app_data().unwrap();
                match service.verify(&signed_security_context) {
                    Ok(security_context) => ok(Self(Some(security_context))),
                    Err(e) => {
                        tracing::warn!(
                            "Failed to verify security context {:?}: {:?}",
                            signed_security_context,
                            e
                        );
                        err(Problem::new(UNAUTHENTICATED).into())
                    }
                }
            }
            // Invalid header = HTTP 401
            Err(_) => err(Problem::new(UNAUTHENTICATED).into()),
        }
    }
}
