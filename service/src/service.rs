use crate::server::Server;
use futures::join;

/// The actual service layer.
pub struct Service {
    /// The HTTP Server.
    server: Server,
}

impl Service {
    /// Create a new instance of the service layer
    ///
    /// # Returns
    /// The service layer, ready to work with.
    pub async fn new() -> Service {
        tracing::info!("Building service");

        let health = crate::health::config::HealthConfig::new();

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
}
