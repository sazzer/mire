use crate::{service::AuthenticationService, ProviderId};
use actix_web::{
    get,
    web::{Data, Path, Query},
    HttpResponse,
};
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
    skip(authentication_service)
)]
pub async fn complete(
    path: Path<(ProviderId,)>,
    query: Query<HashMap<String, String>>,
    authentication_service: Data<AuthenticationService>,
) -> HttpResponse {
    tracing::info!(query = ?query, "Query parameters");
    todo!()
}
