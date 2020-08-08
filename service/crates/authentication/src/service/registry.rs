use super::provider::Provider;
use crate::ProviderId;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of authentication providers that we can use
pub struct Registry {
    /// The map of authentication providers that are known
    pub(crate) providers: HashMap<ProviderId, Arc<dyn Provider>>,
}

impl Registry {
    /// Get the list of all providers that are currently registered
    ///
    /// # Returns
    /// The complete list of all providers that can be used
    pub fn providers(&self) -> Vec<&ProviderId> {
        let mut providers: Vec<&ProviderId> = self.providers.keys().collect();
        providers.sort();
        providers
    }

    /// Get the single provider with the given ID, if there is one.
    /// If the provided ID is unknown then `None` is returned instead.
    ///
    /// # Parameters
    /// - `provider_id` - The ID of the provider to retrieve
    ///
    /// # Returns
    /// The provider that was requested
    pub fn get_provider(&self, provider_id: &ProviderId) -> Option<&Arc<dyn Provider>> {
        self.providers.get(provider_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::provider::{Provider, ProviderMock};
    use assert2::{assert, check};
    use std::str::FromStr;

    #[test]
    fn list_no_providers() {
        let providers = HashMap::new();
        let registry = Registry { providers };

        check!(registry.providers() == Vec::<&ProviderId>::new());
    }

    #[test]
    fn list_some_providers() {
        let mut providers: HashMap<ProviderId, Arc<dyn Provider>> = HashMap::new();
        providers.insert("google".parse().unwrap(), Arc::new(ProviderMock::new()));
        providers.insert("twitter".parse().unwrap(), Arc::new(ProviderMock::new()));
        providers.insert("facebook".parse().unwrap(), Arc::new(ProviderMock::new()));

        let registry = Registry { providers };

        check!(
            registry.providers()
                == vec![
                    &ProviderId::from_str("facebook").unwrap(),
                    &ProviderId::from_str("google").unwrap(),
                    &ProviderId::from_str("twitter").unwrap(),
                ]
        );
    }

    #[test]
    fn get_unknown_provider() {
        let providers: HashMap<ProviderId, Arc<dyn Provider>> = HashMap::new();
        let registry = Registry { providers };

        let provider = registry.get_provider(&"google".parse().unwrap());
        check!(provider.is_none());
    }

    #[test]
    fn get_known_provider() {
        let mut providers: HashMap<ProviderId, Arc<dyn Provider>> = HashMap::new();
        providers.insert("google".parse().unwrap(), Arc::new(ProviderMock::new()));
        let registry = Registry { providers };

        let provider = registry.get_provider(&"google".parse().unwrap());
        assert!(provider.is_some());
        // TODO: Assert it's the correct object.
        // Strictly speaking this isn't needed as there's only one option.
    }
}
