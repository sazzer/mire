use crate::{service::AuthenticationService, ProviderId};
use actix_web::{
    get,
    web::{Data, Path, Query},
    HttpResponse,
};
use mire_authorization::AuthorizationService;
use std::collections::HashMap;

/// HTTP Handler for completing authentication with the desired provider
///
/// # Parameters
/// - `path` - The path parameters including the provider ID
/// - `query` - The set of query parameters needed to complete authentication
/// - `authentication_service` - The Authentiaction Service
///
/// # Returns
/// The HTTP Model for the response
#[get("/authentication/{provider}/complete")]
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication/:provider/complete"),
    skip(authentication_service, authorization_service)
)]
pub async fn complete(
    path: Path<(ProviderId,)>,
    query: Query<HashMap<String, String>>,
    authentication_service: Data<AuthenticationService>,
    authorization_service: Data<AuthorizationService>,
) -> HttpResponse {
    let user = authentication_service
        .complete_authentication(&path.0, &query.0)
        .await
        .unwrap();
    let security_context = authorization_service.generate_security_context(user.identity.id.into());
    let signed_security_context = authorization_service.sign(&security_context);

    HttpResponse::Ok().json(signed_security_context)
}
