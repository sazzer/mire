use std::sync::Arc;

/// Configuration of the healthcheck component.
pub struct HealthConfig {}

impl HealthConfig {
    /// Create a new healthcheck component.
    ///
    /// # Returns
    /// The Healthcheck component Configuration.
    pub fn new() -> Self {
        Self {}
    }

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    pub fn server_config(&self) -> crate::server::FnConfig {
        Arc::new(|c| {
            c.route(
                "/health",
                actix_web::web::get().to(super::endpoints::check_health),
            );
        })
    }
}
