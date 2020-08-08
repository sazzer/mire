#[cfg(test)]
use mockiato::mockable;

use async_trait::async_trait;

/// Trait that represents an Authentication Provider of some kind.
#[async_trait]
#[cfg_attr(test, mockable)]
pub trait Provider {
    async fn a(&self) -> String;
}
