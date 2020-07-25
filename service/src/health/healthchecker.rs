use std::{boxed::Box, error::Error};

/// Trait that components able to report on their health can implement
pub trait Healthchecker {
    /// Check the health of the component
    fn check_health(&self) -> Result<(), Box<Error>>;
}
