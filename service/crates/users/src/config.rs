use super::repository::UsersRepository;
use super::UsersService;
use actix_web::web;
use std::sync::Arc;

/// Configuration of the users component.
pub struct UsersConfig {
    /// The users service to interact with users
    pub service: UsersService,
}

impl UsersConfig {
    /// Construct the users component.
    #[must_use]
    pub const fn new(database: Arc<mire_database::Database>) -> Self {
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

            c.service(
                web::scope("/users")
                    .route("/{id}", web::patch().to(super::endpoints::patch_user))
                    .route("/{id}", web::get().to(super::endpoints::get_user)),
            );
        })
    }
}
