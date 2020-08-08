use chrono::Duration;
use futures::join;
use mire_authentication::config::AuthenticationConfig;
use mire_authorization::{config::AuthorizationConfig, SigningKey};
use mire_database::Database;
use mire_health::{config::HealthConfig, Healthchecker};
use mire_server::{Server, TestResponse};
use mire_users::config::UsersConfig;

use std::sync::Arc;
/// The actual service layer.
pub struct Service {
    /// The HTTP Server.
    server: Server,
}

impl Service {
    /// Create a new instance of the service layer
    ///
    /// # Parameters
    /// - `database_url` - The URL to connect to for the database
    ///
    /// # Returns
    /// The service layer, ready to work with.
    pub async fn new<S>(database_url: S) -> Self
    where
        S: Into<String>,
    {
        tracing::info!("Building service");

        let database = Database::new(database_url).await;
        database
            .check_health()
            .await
            .expect("Database connection is not healthy");

        let _authorization =
            AuthorizationConfig::new(Duration::days(365), SigningKey::new("DummyKey"));

        let _users = UsersConfig::new(database.clone());

        let authentication = AuthenticationConfig::new();

        let mut health = HealthConfig::default();
        health.add_component("db".to_owned(), Arc::new(database));

        let server = Server::new(vec![health.server_config(), authentication.server_config()]);

        Self { server }
    }

    /// Start the service running.
    ///
    /// # Parameters
    /// - `port` - The port to listen on
    pub async fn start(self, port: u16) {
        let http_server = self.server.start(port);
        join!(http_server);
    }

    /// Inject an HTTP Request in to the service and return the response.
    ///
    /// This is strictly for integration testing of the service.
    pub async fn inject(&self, req: actix_http::Request) -> TestResponse {
        self.server.inject(req).await
    }
}
