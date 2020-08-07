mod create;
mod errors;
mod get_by_authentication;
mod get_by_id;
mod parse;

pub use errors::SaveUserError;
use mire_database::Database;

/// Repository for accessing user data
pub struct UsersRepository {
    /// The database connection to use
    database: Database,
}

impl UsersRepository {
    /// Construct a new Users Repository
    ///
    /// # Parameters
    /// - `database` The database connection to use
    ///
    /// # Returns
    /// The Users repository
    pub const fn new(database: Database) -> Self {
        Self { database }
    }
}
