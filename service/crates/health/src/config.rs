use super::{HealthService, Healthchecker};
use actix_web::web;
use std::{collections::HashMap, sync::Arc};

/// Configuration of the healthcheck component.
#[derive(Default)]
pub struct HealthConfig {
    components: HashMap<String, Arc<dyn Healthchecker>>,
}

impl HealthConfig {
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
    #[must_use]
    pub fn server_config(&self) -> mire_server::FnConfig {
        let service = Arc::new(HealthService::new(self.components.clone()));

        Arc::new(move |c| {
            c.data(service.clone());

            c.service(web::resource("/health").route(web::get().to(super::endpoints::get_health)));
        })
    }
}
