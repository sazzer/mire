use super::{access_token::AccessToken, Config};
use crate::service::{AuthenticatedUser, Provider};
use async_trait::async_trait;
use uritemplate::UriTemplate;

/// The default URI to use for starting authentication
const DEFAULT_AUTH_URI: &str = "https://accounts.google.com/o/oauth2/v2/auth{?client_id,redirect_uri,response_type,scope,state}";

/// The default URI to use for fetching authenticated tokens
const DEFAULT_TOKEN_URI: &str = "https://www.googleapis.com/oauth2/v4/token";

/// Implementation of the Provider trait for authenticating with Google
pub struct GoogleProvider {
    /// The HTTP Client to make use of
    client: reqwest::Client,

    /// The Client ID
    client_id: String,
    /// The Client Secret
    client_secret: String,
    /// The URI for Google to redirect to after authentication
    redirect_uri: String,

    /// The URI to redirect to in order to start authentication
    auth_uri: String,
    /// The URI to call to get the authenticated token
    token_uri: String,
}

impl GoogleProvider {
    /// Construct the Google Authentication Provider
    ///
    /// # Parameters
    /// - `config` - The configuration to use for Google
    pub fn new(config: &Config) -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
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
    /// The details of the user that authenticated. If authentication failed for some reason then instead
    /// returns `None` to indicate this fact.
    async fn complete(
        &self,
        params: &std::collections::HashMap<String, String>,
    ) -> Option<AuthenticatedUser> {
        let code = params.get("code")?;
        let params = [
            ("code", code.as_ref()),
            ("client_id", self.client_id.as_ref()),
            ("client_secret", self.client_secret.as_ref()),
            ("redirect_uri", self.redirect_uri.as_ref()),
            ("grant_type", "authorization_code"),
        ];

        let res = self
            .client
            .post(&self.token_uri)
            .form(&params)
            .send()
            .await
            .unwrap();

        let response_status = res.status();

        if !response_status.is_success() {
            let response_body = res.text().await;
            tracing::warn!(body = ?response_body, "Failed to authenticate with Google");
            return None;
        }

        let access_token: AccessToken = res
            .json()
            .await
            .map_err(|e| {
                tracing::debug!(e = ?e, "Failed to parse access token from Google");
                e
            })
            .ok()?;

        tracing::debug!(access_token = ?access_token, "Response from authenticting with Google");

        None
    }
}
