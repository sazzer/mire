use std::collections::HashMap;

/// Possible health states of a single component.
#[derive(Debug)]
pub enum ComponentHealth {
    /// The component is healthy.
    Healthy,
    /// The component is unhealthy.
    Unhealthy(String),
}

impl ComponentHealth {
    /// Indicate if this component is healthy.
    ///
    /// # Returns
    /// True for `ComponentHealth::Healthy`. False for `ComponentHealth::Unhealthy`.
    pub fn is_healthy(&self) -> bool {
        match self {
            ComponentHealth::Healthy => true,
            _ => false,
        }
    }
}

/// Overall health of the system.
#[derive(Debug)]
pub struct SystemHealth {
    /// The health of the various components in the system.
    pub components: HashMap<String, ComponentHealth>,
}

impl SystemHealth {
    /// Indicate if the entire system is healthy.
    ///
    /// # Returns
    /// True if every single component is healthy. False if any of them are unhealthy.
    pub fn is_healthy(&self) -> bool {
        self.components.iter().all(|h| h.1.is_healthy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::*;

    #[test]
    fn test_system_health_empty() {
        let components = HashMap::new();
        let test_subject = SystemHealth { components };

        check!(test_subject.is_healthy() == true);
    }

    #[test]
    fn test_system_health_healthy() {
        let mut components = HashMap::new();
        components.insert("healthy".to_owned(), ComponentHealth::Healthy);
        let test_subject = SystemHealth { components };

        check!(test_subject.is_healthy() == true);
    }

    #[test]
    fn test_system_health_unhealthy() {
        let mut components = HashMap::new();
        components.insert(
            "unhealthy".to_owned(),
            ComponentHealth::Unhealthy("Oops".to_owned()),
        );
        let test_subject = SystemHealth { components };

        check!(test_subject.is_healthy() == false);
    }

    #[test]
    fn test_system_health_mixed() {
        let mut components = HashMap::new();
        components.insert("healthy".to_owned(), ComponentHealth::Healthy);
        components.insert(
            "unhealthy".to_owned(),
            ComponentHealth::Unhealthy("Oops".to_owned()),
        );
        let test_subject = SystemHealth { components };

        check!(test_subject.is_healthy() == false);
    }
}
