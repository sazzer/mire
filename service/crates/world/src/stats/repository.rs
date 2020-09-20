use mire_database::Database;
use std::sync::Arc;

/// Repository for accessing Stat records
pub struct StatsRepository {
    /// The database connection to use
    #[allow(dead_code)]
    database: Arc<Database>,
}

impl StatsRepository {
    /// Construct a new Stats Repository
    ///
    /// # Parameters
    /// - `database` The database connection to use
    ///
    /// # Returns
    /// The Stats repository
    pub const fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
