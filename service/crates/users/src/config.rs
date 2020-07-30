use super::repository::UsersRepository;
use super::UsersService;

/// Configuration of the users component.
pub struct UsersConfig {
    /// The users service to interact with users
    pub service: UsersService,
}

impl UsersConfig {
    /// Construct the users component.
    #[must_use]
    pub const fn new(database: mire_database::Database) -> Self {
        let repository = UsersRepository::new(database);
        let service = UsersService { repository };
        Self { service }
    }
}
