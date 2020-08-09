use crate::{service::Registry, ProviderId};
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
/// - `registry` - The registry of authentication providers
///
/// # Returns
/// The HTTP Model for the response
#[get("/authentication/{provider}")]
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication/:provider"),
    skip(registry)
)]
pub async fn start(path: Path<(ProviderId,)>, registry: Data<Registry>) -> HttpResponse {
    let provider = registry.get_provider(&path.0);

    if let Some(_provider) = provider {
        let mut response = HttpResponse::Found();
        response.set(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(3600),
        ]));
        response.set_header("location", "http://www.google.com");
        response.cookie(
            Cookie::build("mire_authentication_nonce", "someNonce")
                .http_only(true)
                .finish(),
        );
        response.cookie(
            Cookie::build("mire_authentication_provider", path.0.0.clone())
                .http_only(true)
                .finish(),
        );
        response.finish()
    } else {
        HttpResponse::NotFound()
            .set(CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::MaxAge(3600),
            ]))
            .finish()
    }
}
