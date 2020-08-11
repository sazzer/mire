use crate::{service::AuthenticationService, ProviderId, StartAuthentication};
use actix_http::http::header::{CacheControl, CacheDirective};
use actix_web::{
    cookie::Cookie,
    get,
    web::{Data, Path},
    HttpResponse,
};

/// HTTP Handler for starting authentication with the desired provider
///
/// # Parameters
/// - `path` - The path parameters including the provider ID
/// - `authentication_service` - The Authentiaction Service
///
/// # Returns
/// The HTTP Model for the response
#[get("/authentication/{provider}")]
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication/:provider"),
    skip(authentication_service)
)]
pub async fn start(
    path: Path<(ProviderId,)>,
    authentication_service: Data<AuthenticationService>,
) -> HttpResponse {
    match authentication_service.start_authentication(&path.0) {
        Ok(StartAuthentication { redirect_uri, nonce }) => HttpResponse::Found()
            .set_header("location", redirect_uri)
            .set(CacheControl(vec![CacheDirective::NoCache]))
            .cookie(
                Cookie::build("mire_authentication_provider", path.0.0.clone())
                    .http_only(true)
                    .finish(),
            )
            .cookie(
                Cookie::build("mire_authentication_nonce", nonce)
                    .http_only(true)
                    .finish(),
            )
            .finish(),
        Err(_) => HttpResponse::NotFound()
            .set(CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::MaxAge(3600),
            ]))
            .finish(),
    }
}
