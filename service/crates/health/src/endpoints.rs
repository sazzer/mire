use super::{ComponentHealth, HealthService, SystemHealth};
use actix_http::{http::StatusCode, Error, Response};
use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};
use futures::future::{ok, Ready};
use serde::Serialize;
use std::collections::HashMap;

/// HTTP Handler for checking the health of the system.
///
/// # Parameters
/// - `health_service` - The Health Service to get the system health with
///
/// # Returns
/// The HTTP Model for the response
#[get("/health")]
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/health"),
    skip(health_service)
)]
pub async fn get_health(health_service: Data<HealthService>) -> impl Responder {
    let system_health = health_service.check_health().await;

    SystemHealthModel::from(system_health)
}

/// HTTP Representation of the health of a single component
#[derive(Serialize)]
struct ComponentHealthModel {
    /// Whether the component is healthy or not
    pub healthy: bool,
    /// A message about the component if it was unhealthy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl From<ComponentHealth> for ComponentHealthModel {
    /// Convert the internal `ComponentHealth` instance into the HTTP Response.
    fn from(health: ComponentHealth) -> Self {
        ComponentHealthModel {
            healthy: health.is_healthy(),
            message: match health {
                ComponentHealth::Healthy => None,
                ComponentHealth::Unhealthy(e) => Some(e),
            },
        }
    }
}

/// HTTP Representation of the health of the entire system
#[derive(Serialize)]
struct SystemHealthModel {
    /// Whether the system is healthy or not
    pub healthy: bool,
    /// The individual components that make up the system.
    pub components: HashMap<String, ComponentHealthModel>,
}

impl From<SystemHealth> for SystemHealthModel {
    /// Convert the internal `SystemHealth` instance into the HTTP Response.
    fn from(health: SystemHealth) -> Self {
        SystemHealthModel {
            healthy: health.is_healthy(),
            components: health
                .components
                .into_iter()
                .map(|(name, component)| (name, component.into()))
                .collect(),
        }
    }
}

impl Responder for SystemHealthModel {
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    /// Generate an HTTP Response for the System Health.
    ///
    /// The status code is either `StatusCode::OK` for a healthy system or `StatusCode::SERVICE_UNAVAILABLE`
    /// for an unhealthy system, and the body is the JSON version of this type.
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let status_code = if self.healthy {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        };

        let response = HttpResponse::build(status_code).json(self);

        ok(response)
    }
}
