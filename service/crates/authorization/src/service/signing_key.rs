use biscuit::jws::Secret;

/// The key with which to sign the security context
#[derive(Clone)]
pub struct SigningKey(String);

impl SigningKey {
    /// Construct a new signing key from the provided value
    ///
    /// # Parameters
    /// - `key` The signing key to wrap
    pub fn new<S>(key: S) -> Self
    where
        S: Into<String>,
    {
        Self(key.into())
    }
}

impl From<SigningKey> for Secret {
    fn from(signing_key: SigningKey) -> Self {
        Self::bytes_from_str(&signing_key.0)
    }
}
