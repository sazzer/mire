use actix_cors::Cors;
use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use std::{ops::Deref, sync::Arc};

/// A function that is able to contribute configuration to the Actix server when it is being constructed
pub type FnConfig = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;

/// Representation of the HTTP Server
pub struct Server {
    configs: Vec<FnConfig>,
}

impl Server {
    /// Create a new instance of the server
    pub fn new(configs: Vec<FnConfig>) -> Server {
        Server { configs }
    }

    /// Start the server listening on the given port
    pub async fn start(self, port: u16) {
        let address = format!("0.0.0.0:{}", port);
        tracing::info!(address = ?address, "Starting web server");

        let configs = self.configs.clone();
        HttpServer::new(move || {
            let configs = configs.clone();

            let mut app = App::new()
                .wrap(Logger::default())
                .wrap(Cors::new().finish());

            for config in &configs {
                app = app.configure(config.deref());
            }

            app
        })
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
