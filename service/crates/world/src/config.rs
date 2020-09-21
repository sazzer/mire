use crate::stats::{repository::StatsRepository, StatsService};
use std::sync::Arc;

/// Configuration of the world component.
pub struct WorldConfig {
    pub stats_service: Arc<StatsService>,
}

impl WorldConfig {
    /// Construct the world component.
    #[must_use]
    pub fn new(database: Arc<mire_database::Database>) -> Self {
        Self {
            stats_service: Arc::new(StatsService {
                repository: StatsRepository::new(database),
            }),
        }
    }

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    #[must_use]
    pub fn server_config(self) -> mire_server::FnConfig {
        let stats_service = self.stats_service;

        Arc::new(move |c| {
            c.data(stats_service.clone());
        })
    }
}
