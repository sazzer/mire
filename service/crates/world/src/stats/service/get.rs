use super::StatsService;
use crate::{Stat, StatId};

impl StatsService {
    /// Get the one stat with the provided ID.
    ///
    /// # Parameters
    /// - `stat_id` - The ID of the Stat to get.
    ///
    /// # Returns
    /// The single stat, or `None` if it wasn't found.
    #[tracing::instrument(skip(self))]
    pub async fn get(&self, stat_id: &StatId) -> Option<Stat> {
        self.repository.get(stat_id).await
    }
}
