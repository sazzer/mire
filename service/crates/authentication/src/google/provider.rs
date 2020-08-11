use super::Config;
use crate::service::Provider;
use async_trait::async_trait;
use uritemplate::UriTemplate;

/// The default URI to use for starting authentication
const DEFAULT_AUTH_URI: &str = "https://accounts.google.com/o/oauth2/v2/auth{?client_id,redirect_uri,response_type,scope,state}";

/// The default URI to use for fetching authenticated tokens
const DEFAULT_TOKEN_URI: &str = "https://www.googleapis.com/oauth2/v4/token";

/// Implementation of the Provider trait for authenticating with Google
pub struct GoogleProvider {
    /// The Client ID
    pub client_id: String,
    /// The Client Secret
    pub client_secret: String,
    /// The URI for Google to redirect to after authentication
    pub redirect_uri: String,

    /// The URI to redirect to in order to start authentication
    pub auth_uri: String,
    /// The URI to call to get the authenticated token
    pub token_uri: String,
}

impl GoogleProvider {
    /// Construct the Google Authentication Provider
    ///
    /// # Parameters
    /// - `config` - The configuration to use for Google
    pub fn new(config: &Config) -> Self {
        Self {
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
            redirect_uri: config.redirect_uri.clone(),
            auth_uri: config
                .auth_uri
                .clone()
                .unwrap_or_else(|| DEFAULT_AUTH_URI.to_owned()),
            token_uri: config
                .token_uri
                .clone()
                .unwrap_or_else(|| DEFAULT_TOKEN_URI.to_owned()),
        }
    }
}

#[async_trait]
impl Provider for GoogleProvider {
    /// Generate the details needed to redirect the user to authenticate with this provider
    ///
    /// # Parameters
    /// - `nonce` - A generated nonce unique to this request
    ///
    /// # Returns
    /// The URL to redirect the user to in order to start authentication
    fn start(&self, nonce: &str) -> String {
        tracing::debug!("Generating URI to authenticate against Google");

        let uri = UriTemplate::new(&self.auth_uri)
            .set("client_id", self.client_id.clone())
            .set("redirect_uri", self.redirect_uri.clone())
            .set("state", nonce)
            .set("response_type", "code")
            .set("scope", "openid email profile")
            .build();

        tracing::debug!(uri = ?uri, state = ?nonce, "Generated URI to authenticate against Google");

        uri
    }

    /// Complete authentication against the provider and return details of the user that authenticated
    ///
    /// # Parameters
    /// - `params` - The parameters received from the callback from the provider
    ///
    /// # Returns
    /// The details of the user that authenticated
    ///
    /// # Errors
    /// If an error occurs authenticating against the provider
    async fn complete(&self, params: &std::collections::HashMap<String, String>) {
        let _code = params.get("code");

        todo!()
    }
}
