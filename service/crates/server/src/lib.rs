use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::ServiceConfig, App, HttpServer};
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
    #[must_use]
    pub fn new(configs: Vec<FnConfig>) -> Self {
        Self { configs }
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
                .wrap(Cors::new().allowed_header(header::ETAG).finish());

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

        let status = response.status();
        let headers = response.headers().clone();
        let body = actix_web::test::read_body(response).await;

        TestResponse {
            status,
            headers,
            body,
        }
    }
}

/// Representation of the response to injecting a test request
pub struct TestResponse {
    /// The status code
    pub status: actix_http::http::StatusCode,
    /// The set of headers
    pub headers: actix_http::http::HeaderMap,
    /// The response body
    pub body: bytes::Bytes,
}

impl TestResponse {
    /// Get the value of the header with the given name
    ///
    /// # Parameters
    /// - `name` - The name of the header
    ///
    /// # Returns
    /// The header, if present. `None` if it wasn't present.
    pub fn header<S>(&self, name: S) -> Option<&actix_http::http::HeaderValue>
    where
        S: Into<String>,
    {
        self.headers.get(name.into())
    }

    /// Convert the response body to JSON
    ///
    /// # Returns
    /// The body of the response, converted to a Serde JSON object
    ///
    /// # Errors
    /// Any errors from deserializing the response
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::error::Error> {
        serde_json::from_slice(&self.body)
    }
}
