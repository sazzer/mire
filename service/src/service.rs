use crate::server::Server;

/// The actual service layer
pub struct Service {
    server: Server,
}

impl Service {
    /// Create a new instance of the service layer
    pub async fn new() -> Service {
        tracing::info!("Building service");

        let server = Server::new(vec![]);

        Service { server: server }
    }

    /// Start the service running
    pub async fn start(self, port: u16) {
        self.server.start(port).await;
    }
}
