/// The key with which to sign the security context
pub struct SigningKey(String);

/// The service layer for managing authorization of principals
pub struct AuthorizationService {
    /// The key with which to sign security contexts
    #[allow(dead_code)]
    pub(super) signing_key: SigningKey,
}

impl AuthorizationService {
    /// Create a new Authorization Service to authorize principals for access to services
    ///
    /// # Parameters
    /// - `signing_key` - The key with which to sign security contexts
    ///
    /// # Returns
    /// The Authorization Service
    #[must_use]
    pub const fn new(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }
}
