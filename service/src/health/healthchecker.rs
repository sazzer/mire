use std::{boxed::Box, error::Error};

/// Trait that components able to report on their health can implement.
pub trait Healthchecker {
    /// Check the health of the component.
    ///
    /// # Returns
    /// If the component is healthy then returns `Ok(())`.
    /// If the component is unhealthy then returns `Err()` containing the reason why the component is unhealthy.
    fn check_health(&self) -> Result<(), Box<dyn Error>>;
}
