use crate::server::Server;
use futures::join;

/// The actual service layer
pub struct Service {
    server: Server,
}

impl Service {
    /// Create a new instance of the service layer
    pub async fn new() -> Service {
        tracing::info!("Building service");

        let health = crate::health::config::HealthConfig::new();

        let server = Server::new(vec![health.server_config()]);

        Service { server: server }
    }

    /// Start the service running
    pub async fn start(self, port: u16) {
        let http_server = self.server.start(port);
        join!(http_server);
    }
}
