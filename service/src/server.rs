use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

/// Representation of the HTTP Server
pub struct Server {}

impl Server {
    /// Create a new instance of the server
    pub fn new() -> Server {
        Server {}
    }

    /// Start the server listening on the given port
    pub async fn start(self, port: u16) {
        let address = format!("0.0.0.0:{}", port);
        tracing::info!(address = ?address, "Starting web server");

        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .wrap(Cors::new().finish())
        })
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
