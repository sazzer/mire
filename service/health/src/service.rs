use super::{model::ComponentHealth, model::SystemHealth, Healthchecker};
use std::{collections::HashMap, sync::Arc};

/// Service layer for checking the health of the system.
#[derive(Clone)]
pub struct HealthService {
    /// The map of components to check the health of.
    components: HashMap<String, Arc<dyn Healthchecker>>,
}

impl HealthService {
    /// Create a new instance of the health service.
    ///
    /// # Parameters
    /// - `components` - The map of components to check the health of.
    ///
    /// # Returns
    /// The health service
    #[must_use]
    pub fn new(components: HashMap<String, Arc<dyn Healthchecker>>) -> Self {
        Self { components }
    }

    /// Actually check the health of the system.
    ///
    /// # Returns
    /// The overall system health
    #[tracing::instrument(skip(self))]
    pub async fn check_health(&self) -> SystemHealth {
        tracing::debug!("Checking system health");

        let mut components = HashMap::new();
        for (name, component) in &self.components {
            let result = component.check_health().await;

            let component_health = match result {
                Ok(_) => ComponentHealth::Healthy,
                Err(e) => ComponentHealth::Unhealthy(e.to_string()),
            };

            tracing::debug!(name = ?name, status = ?component_health, "Component health");
            components.insert(name.clone(), component_health);
        }

        SystemHealth { components }
    }
}
