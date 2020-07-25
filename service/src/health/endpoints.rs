use actix_web::{HttpResponse, Responder};

/// HTTP Handler for checking the health of the system.
pub async fn check_health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
