mod create;
mod errors;
mod get_by_authentication;
mod get_by_id;
mod parse;
mod update;

pub use errors::SaveUserError;
use mire_database::Database;
use std::sync::Arc;

/// Repository for accessing user data
pub struct UsersRepository {
    /// The database connection to use
    database: Arc<Database>,
}

impl UsersRepository {
    /// Construct a new Users Repository
    ///
    /// # Parameters
    /// - `database` The database connection to use
    ///
    /// # Returns
    /// The Users repository
    pub const fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
