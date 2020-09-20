use super::StatsRepository;
use crate::Stat;

impl StatsRepository {
    /// List all of the stats that exist in the data store.
    /// Note that this does not support any filtering or sorting, and just returns them as they come.
    ///
    /// # Returns
    /// The collection of all stats in the repository.
    #[allow(dead_code)]
    pub async fn list(&self) -> Vec<Stat> {
        tracing::debug!("Listing stats");
        let conn = self.database.checkout().await.unwrap();

        let stats = conn
            .query("SELECT * FROM stats ORDER BY stat_id", &[])
            .await
            .unwrap()
            .into_iter()
            .map(Stat::from)
            .collect();

        tracing::debug!("Found stats: {:?}", stats);

        stats
    }
}
