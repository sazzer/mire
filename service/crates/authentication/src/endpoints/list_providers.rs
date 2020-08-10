use crate::service::AuthenticationService;
use actix_http::http::header::{CacheControl, CacheDirective};
use actix_web::{get, web::Data, HttpResponse};

/// HTTP Handler for getting the list of authentication providers.
///
/// # Parameters
/// - `authentication_service` - The Authentiaction Service
///
/// # Returns
/// The HTTP Model for the response
#[get("/authentication")]
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication"),
    skip(authentication_service)
)]
pub async fn list_providers(authentication_service: Data<AuthenticationService>) -> HttpResponse {
    let providers = authentication_service.providers();

    HttpResponse::Ok()
        .set(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(3600),
        ]))
        .json(providers)
}
