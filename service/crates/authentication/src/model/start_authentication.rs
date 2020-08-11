/// Details needed to start authentication with a provider
#[derive(Debug)]
pub struct StartAuthentication {
    /// The URI to redirect the user to in order to start authentication
    pub redirect_uri: String,
    /// A nonce for this request to prove that the same user has come back
    pub nonce: String,
}
