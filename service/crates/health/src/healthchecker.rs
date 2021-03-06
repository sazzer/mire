use async_trait::async_trait;
use std::{boxed::Box, error::Error};

/// Trait that components able to report on their health can implement.
#[async_trait]
pub trait Healthchecker: Send + Sync {
    /// Check the health of the component.
    ///
    /// # Returns
    /// If the component is healthy then returns `Ok(())`.
    /// If the component is unhealthy then returns `Err()` containing the reason why the component is unhealthy.
    async fn check_health(&self) -> Result<(), Box<dyn Error>>;
}
