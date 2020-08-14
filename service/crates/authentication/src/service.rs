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

use mire_authorization::AuthorizationService;
use mire_users::UsersService;

/// The authentication service that manages all authentication details
#[derive(Clone)]
pub struct AuthenticationService {
    registry: Registry,
    users_service: UsersService,
    authorization_service: AuthorizationService,
}

impl AuthenticationService {
    /// Construct a new Authentication Service
    ///
    /// # Parameters
    /// - `registry` - The registry of authentication providers
    /// - `users_service` - The users service for interacting with user records
    /// - `authorization_service` - The authorization service for generating security contexts
    pub const fn new(
        registry: Registry,
        users_service: UsersService,
        authorization_service: AuthorizationService,
    ) -> Self {
        Self {
            registry,
            users_service,
            authorization_service,
        }
    }
}
