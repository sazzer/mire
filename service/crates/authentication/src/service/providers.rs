use super::AuthenticationService;
use crate::ProviderId;

impl AuthenticationService {
    /// Get the list of all providers that are currently registered
    ///
    /// # Returns
    /// The complete list of all providers that can be used
    pub fn providers(&self) -> Vec<&ProviderId> {
        let mut providers: Vec<&ProviderId> = self.registry.providers();
        providers.sort();
        providers
    }
}
