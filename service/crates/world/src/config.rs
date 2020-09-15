use std::sync::Arc;

/// Configuration of the world component.
pub struct WorldConfig {}

impl WorldConfig {
    /// Construct the world component.
    #[must_use]
    #[allow(clippy::needless_pass_by_value, clippy::missing_const_for_fn)]
    pub fn new(_database: mire_database::Database) -> Self {
        Self {}
    }

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn server_config(&self) -> mire_server::FnConfig {
        Arc::new(move |_c| {})
    }
}
