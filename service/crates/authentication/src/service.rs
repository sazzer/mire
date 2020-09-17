mod authenticated_user;
mod complete;
mod provider;
mod providers;
mod registry;
mod start;

pub use authenticated_user::AuthenticatedUser;
pub use provider::Provider;
pub use registry::Registry;
pub use start::StartAuthenticationError;

use mire_users::UsersService;
use std::sync::Arc;

/// The authentication service that manages all authentication details
#[derive(Clone)]
pub struct AuthenticationService {
    registry: Registry,
    users_service: Arc<UsersService>,
}

impl AuthenticationService {
    /// Construct a new Authentication Service
    ///
    /// # Parameters
    /// - `registry` - The registry of authentication providers
    /// - `users_service` - The users service for interacting with user records
    pub const fn new(registry: Registry, users_service: Arc<UsersService>) -> Self {
        Self {
            registry,
            users_service,
        }
    }
}
