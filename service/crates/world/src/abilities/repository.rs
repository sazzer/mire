use mire_database::Database;
use std::sync::Arc;

/// Repository for accessing Ability records
pub struct AbilitiesRepository {
    /// The database connection to use
    #[allow(dead_code)]
    database: Arc<Database>,
}

impl AbilitiesRepository {
    /// Construct a new Abilities Repository
    ///
    /// # Parameters
    /// - `database` The database connection to use
    ///
    /// # Returns
    /// The Abilities repository
    pub const fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
