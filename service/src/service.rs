use crate::database::Database;
use crate::{
    health::config::HealthConfig,
    server::{Server, TestResponse},
};
use futures::join;
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
    pub async fn new<S>(database_url: S) -> Service
    where
        S: Into<String>,
    {
        tracing::info!("Building service");

        let database = Database::new(database_url).await;

        let mut health = HealthConfig::new();
        health.add_component("db".to_owned(), Arc::new(MockHealthcheck {}));

        let server = Server::new(vec![health.server_config()]);

        Service { server: server }
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

struct MockHealthcheck {}

#[async_trait::async_trait]
impl crate::health::Healthchecker for MockHealthcheck {
    async fn check_health(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
