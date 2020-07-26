use actix_cors::Cors;
use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use std::{ops::Deref, sync::Arc};

/// A function that is able to contribute configuration to the Actix server when it is being constructed.
pub type FnConfig = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;

/// Wrapper around the HTTP Server.
pub struct Server {
    /// The set of configuration lambdas that can contribute to the HTTP Server.
    configs: Vec<FnConfig>,
}

impl Server {
    /// Create a new instance of the server
    ///
    /// # Parameters
    /// - `configs` - The set of configuration lambdas that can contribute to the HTTP Server.
    ///
    /// # Returns
    /// The wrapper around the HTTP Server.
    pub fn new(configs: Vec<FnConfig>) -> Server {
        Server { configs }
    }

    /// Start the server listening on the given port.
    ///
    /// # Parameters
    /// - `port` - The port to listen on
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

    /// Inject an HTTP Request in to the service and return the response.
    ///
    /// This is strictly for integration testing of the service.
    pub async fn inject(&self, req: actix_http::Request) -> TestResponse {
        let configs = self.configs.clone();

        let mut app = App::new();

        for config in &configs {
            app = app.configure(config.deref());
        }

        let mut test_service = actix_web::test::init_service(app).await;
        let response = actix_web::test::call_service(&mut test_service, req).await;

        TestResponse {
            status: response.status(),
        }
    }
}

pub struct TestResponse {
    pub status: actix_http::http::StatusCode,
}
