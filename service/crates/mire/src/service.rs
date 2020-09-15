use crate::Config;
use chrono::Duration;
use futures::join;
use mire_authentication::config::AuthenticationConfig;
use mire_authorization::{config::AuthorizationConfig, AuthorizationService, SigningKey};
use mire_database::Database;
use mire_health::{config::HealthConfig, Healthchecker};
use mire_server::{Server, TestResponse};
use mire_users::config::UsersConfig;
use mire_world::config::WorldConfig;

use std::sync::Arc;
/// The actual service layer.
pub struct Service {
    /// The HTTP Server.
    server: Server,
    /// The Authorization Service, needed for testing.
    authorization_service: AuthorizationService,
}

impl Service {
    /// Create a new instance of the service layer
    ///
    /// # Parameters
    /// - `config` - The configuration of the service
    ///
    /// # Returns
    /// The service layer, ready to work with.
    pub async fn new(config: Config) -> Self {
        tracing::info!("Building service");

        let database = Database::new(config.database_url).await;
        database
            .check_health()
            .await
            .expect("Database connection is not healthy");

        let authorization =
            AuthorizationConfig::new(Duration::days(365), SigningKey::new("DummyKey"));

        let users = UsersConfig::new(database.clone());

        let mut authentication = AuthenticationConfig::new(users.service.clone());
        if let Some(google) = config.google_config {
            authentication.with_google(&google);
        }

        let world = WorldConfig::new(database.clone());

        let mut health = HealthConfig::default();
        health.add_component("db".to_owned(), Arc::new(database));

        let server = Server::new(vec![
            health.server_config(),
            authorization.server_config(),
            authentication.server_config(),
            users.server_config(),
            world.server_config(),
        ]);

        Self {
            server,
            authorization_service: authorization.service,
        }
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

    /// Generate a signed security context that can be used to access the service.
    ///
    /// # Parameters
    /// - `principal` - The Principal whom the security context represents
    ///
    /// # Returns
    /// The security context
    #[must_use]
    pub fn generate_access_token(
        &self,
        principal: mire_authorization::PrincipalId,
    ) -> mire_authorization::SignedSecurityContext {
        let security_context = self
            .authorization_service
            .generate_security_context(principal);
        self.authorization_service.sign(&security_context)
    }
}
