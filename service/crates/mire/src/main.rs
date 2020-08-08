use config::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// Representation of the application settings that will be loaded from the environment
#[derive(Debug, Deserialize)]
struct Settings {
    /// The port on which the HTTP server should listen on
    pub port: Option<u16>,
    /// The URL to use to connect to the database
    pub database_url: Option<String>,

    /// The Client ID for Google Authentication
    pub google_client_id: Option<String>,
    /// The Client Secret for Google Authentication
    pub google_client_secret: Option<String>,
    /// The URI for Google to redirect to after authentication
    pub google_redirect_uri: Option<String>,

    /// The URI to redirect to in order to start authentication with Google
    pub google_auth_uri: Option<String>,
    /// The URI to call to get the authenticated token from Google
    pub google_token_uri: Option<String>,
}

impl Default for Settings {
    /// Construct the settings from the environment
    ///
    /// # Returns
    /// The Settings object, loaded from the environment variables
    fn default() -> Self {
        let mut s = Config::new();
        s.merge(Environment::default())
            .expect("Failed to load environment properties");

        s.try_into().expect("Failed to build settings from config")
    }
}

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let settings = Settings::default();

    tracing::debug!(settings = ?settings, "Application settings");

    let google_config = if let (Some(client_id), Some(client_secret), Some(redirect_uri)) = (
        settings.google_client_id,
        settings.google_client_secret,
        settings.google_redirect_uri,
    ) {
        Some(mire_authentication::google::Config {
            client_id,
            client_secret,
            redirect_uri,
            auth_uri: settings.google_auth_uri,
            token_uri: settings.google_token_uri,
        })
    } else {
        None
    };

    let config = mire_lib::Config {
        database_url: settings.database_url.expect("No database URL provided"),
        google_config,
    };

    let service = mire_lib::Service::new(config).await;
    service.start(settings.port.unwrap_or(8000)).await;
}
