use super::repository::UsersRepository;
use super::UsersService;
use std::sync::Arc;

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

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    #[must_use]
    pub fn server_config(&self) -> mire_server::FnConfig {
        let service = self.service.clone();
        Arc::new(move |c| {
            c.data(service.clone());

            c.service(super::endpoints::get_user);
        })
    }
}
