use super::StatsService;
use crate::Stat;
use mire_model::Page;

impl StatsService {
    /// Get all of the stats
    ///
    /// # Returns
    /// The entire set of stats in the system.
    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Page<Stat> {
        self.repository.list().await
    }
}
