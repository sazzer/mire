mod get;
mod list;

use super::repository::StatsRepository;

/// Service layer for interacting with Stat data
pub struct StatsService {
    /// The repository for accessing stat data
    #[allow(dead_code)]
    pub(crate) repository: StatsRepository,
}
