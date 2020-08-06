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

    let service =
        mire_lib::Service::new(settings.database_url.expect("No database URL provided")).await;
    service.start(settings.port.unwrap_or(8000)).await;
}