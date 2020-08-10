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
    if let Some(provider) = registry.get_provider(&path.0) {
        let authentication = provider.start();

        let mut response = HttpResponse::Found();
        response.set(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(3600),
        ]));
        response.set_header("location", authentication.redirect_uri);
        response.cookie(
            Cookie::build("mire_authentication_provider", path.0.0.clone())
                .http_only(true)
                .finish(),
        );

        if let Some(nonce) = authentication.nonce {
          response.cookie(
            Cookie::build("mire_authentication_nonce", nonce)
                .http_only(true)
                .finish(),
          );
        }

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
