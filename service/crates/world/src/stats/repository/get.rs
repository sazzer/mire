use super::StatsRepository;
use crate::{Stat, StatId};

impl StatsRepository {
    /// Get the one stat from the data store with the provided ID.
    ///
    /// # Parameters
    /// - `stat_id` - The ID of the Stat to get.
    ///
    /// # Returns
    /// The single stat, or `None` if it wasn't found.
    pub async fn get(&self, stat_id: &StatId) -> Option<Stat> {
        tracing::debug!(stat_id = ?stat_id, "Loading stat by ID");

        let conn = self.database.checkout().await.unwrap();

        let stat = conn
            .query_opt("SELECT * FROM stats WHERE stat_id = $1", &[&stat_id])
            .await
            .unwrap()
            .map(Stat::from);

        tracing::debug!("Found stat: {:?}", stat);

        stat
    }
}
