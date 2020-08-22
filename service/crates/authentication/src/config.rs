use crate::service::Registry;
use mire_users::UsersService;
use std::sync::Arc;

/// Configuration of the authentication component.
pub struct AuthenticationConfig {
    /// The registry of authentication providers
    pub(crate) registry: Registry,

    /// The Users Service, to look up and register users
    users_service: UsersService,
}

impl AuthenticationConfig {
    /// Construct the authentication component.
    #[must_use]
    pub fn new(users_service: UsersService) -> Self {
        let registry = Registry::default();
        Self {
            registry,
            users_service,
        }
    }

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    #[must_use]
    pub fn server_config(&self) -> mire_server::FnConfig {
        let service = crate::service::AuthenticationService::new(
            self.registry.clone(),
            self.users_service.clone(),
        );

        Arc::new(move |c| {
            c.data(service.clone());

            c.service(super::endpoints::list_providers);
            c.service(super::endpoints::start);
            c.service(super::endpoints::complete);
        })
    }
}