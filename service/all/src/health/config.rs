use super::{HealthService, Healthchecker};
use std::{collections::HashMap, sync::Arc};

/// Configuration of the healthcheck component.
pub struct HealthConfig {
    components: HashMap<String, Arc<dyn Healthchecker>>,
}

impl HealthConfig {
    /// Create a new healthcheck component.
    ///
    /// # Returns
    /// The Healthcheck component Configuration.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// Add a new component to the healthchecker so that we can check the health of it.
    ///
    /// # Parameters
    /// - `name` - The name of the component. Must be unique.
    /// - `component` - The actual component.
    pub fn add_component<S>(&mut self, name: S, component: Arc<dyn Healthchecker>)
    where
        S: Into<String>,
    {
        self.components.insert(name.into(), component);
    }

    /// Return a configuration function to contribute to the HTTP Server.
    ///
    /// # Returns
    /// The lambda to register details with the HTTP Server.
    pub fn server_config(&self) -> mire_server::FnConfig {
        let service = HealthService::new(self.components.clone());

        Arc::new(move |c| {
            c.data(service.clone());

            c.service(super::endpoints::get_health);
        })
    }
}
