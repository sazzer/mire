use crate::service::Registry;
use actix_http::http::header::{CacheControl, CacheDirective};
use actix_web::{get, web::Data, HttpResponse};

/// HTTP Handler for getting the list of authentication providers.
///
/// # Parameters
/// - `registry` - The registry of authentication providers
///
/// # Returns
/// The HTTP Model for the response
#[get("/authentication")]
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication"),
    skip(registry)
)]
pub async fn list_providers(registry: Data<Registry>) -> HttpResponse {
    let providers = registry.providers();

    HttpResponse::Ok()
        .set(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(3600),
        ]))
        .json(providers)
}
