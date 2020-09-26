use super::model::StatModel;
use crate::{StatId, StatsService};
use actix_web::web::{Data, Path};
use mire_problem::{Problem, NOT_FOUND};
use std::sync::Arc;

/// HTTP Handler for getting the details of a Stat by ID
///
/// # Parameters
/// - `path` - The Path containing the ID of the stat
/// - `stats_service` - The Stats Service
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/stats/:id"),
    skip(stats_service)
)]
pub async fn get_stat(
    path: Path<StatId>,
    stats_service: Data<Arc<StatsService>>,
) -> Result<StatModel, Problem> {
    let stat_id = &path.0;

    let stat = stats_service.get(stat_id).await;
    tracing::debug!("Found stat: {:?}", stat);

    stat.map(StatModel::from)
        .ok_or_else(|| Problem::new(NOT_FOUND))
}
