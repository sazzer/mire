use async_trait::async_trait;

/// Trait that represents an Authentication Provider of some kind.
#[async_trait]
pub trait Provider {}
