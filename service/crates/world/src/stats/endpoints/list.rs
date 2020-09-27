use super::model::StatModel;
use crate::StatsService;
use actix_web::web::Data;
use mire_http::pagination::PageModel;
use std::sync::Arc;

/// HTTP Handler for getting the details of all Stats
///
/// # Parameters
/// - `stats_service` - The Stats Service
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(fields(http_method = "GET", http_path = "/stats"), skip(stats_service))]
pub async fn list_stats(stats_service: Data<Arc<StatsService>>) -> PageModel<StatModel> {
    let stats = stats_service.list().await;
    tracing::debug!("Found stats: {:?}", stats);

    stats.into()
}
