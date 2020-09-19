use crate::abilities::{repository::AbilitiesRepository, AbilitiesService};
use std::sync::Arc;

/// Configuration of the world component.
pub struct WorldConfig {
    abilities_service: Arc<AbilitiesService>,
}

impl WorldConfig {
    /// Construct the world component.
    #[must_use]
    pub fn new(database: Arc<mire_database::Database>) -> Self {
        Self {
            abilities_service: Arc::new(AbilitiesService {
                repository: AbilitiesRepository::new(database),
            }),
        }
    }

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    #[must_use]
    pub fn server_config(self) -> mire_server::FnConfig {
        let abilities_service = self.abilities_service;

        Arc::new(move |c| {
            c.data(abilities_service.clone());
        })
    }
}
