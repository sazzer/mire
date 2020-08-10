mod provider;
mod providers;
mod registry;

pub use provider::Provider;
pub use registry::Registry;

/// The authentication service that manages all authentication details
#[derive(Clone)]
pub struct AuthenticationService {
    registry: Registry,
}

impl AuthenticationService {
    /// Construct a new Authentication Service
    ///
    /// # Parameters
    /// - `registry` - The registry of authentication providers
    pub const fn new(registry: Registry) -> Self {
        Self { registry }
    }
}
