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

/// The authentication service that manages all authentication details
#[derive(Clone)]
pub struct AuthenticationService {
    registry: Registry,
    users_service: UsersService,
}

impl AuthenticationService {
    /// Construct a new Authentication Service
    ///
    /// # Parameters
    /// - `registry` - The registry of authentication providers
    pub const fn new(registry: Registry, users_service: UsersService) -> Self {
        Self {
            registry,
            users_service,
        }
    }
}
