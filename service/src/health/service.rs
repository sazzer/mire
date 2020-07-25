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
    pub fn new(components: HashMap<String, Arc<dyn Healthchecker>>) -> Self {
        Self { components }
    }

    /// Actually check the health of the system.
    ///
    /// # Returns
    /// The overall system health
    pub fn check_health(&self) -> SystemHealth {
        let components = self
            .components
            .iter()
            .map(|(name, component)| {
                let result = component.check_health();

                let component_health = match result {
                    Ok(_) => ComponentHealth::Healthy,
                    Err(e) => ComponentHealth::Unhalthy(e.to_string()),
                };

                (name.clone(), component_health)
            })
            .collect();

        SystemHealth { components }
    }
}
