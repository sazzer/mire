use super::HealthService;
use actix_web::{get, web::Data, HttpResponse, Responder};

/// HTTP Handler for checking the health of the system.
#[get("/health")]
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/health"),
    skip(health_service)
)]
pub async fn get_health(health_service: Data<HealthService>) -> impl Responder {
    health_service.check_health().await;

    HttpResponse::Ok().body("Hello world!")
}
