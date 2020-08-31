use crate::service::UsersService;
use crate::UserId;
use actix_web::{
    get,
    web::{Data, Path},
};
use mire_problem::{Problem, NOT_FOUND};

/// HTTP Handler for getting the details of a User by ID.
///
/// # Parameters
/// - `users_service` - The Users Service
///
/// # Returns
/// The HTTP Model for the response
#[get("/users/{id}")]
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/users/:id"),
    skip(users_service)
)]
pub async fn get_user(path: Path<(UserId,)>, users_service: Data<UsersService>) -> Problem {
    let user = users_service.get_by_id(&path.0).await;
    tracing::debug!("Found user: {:?}", user);

    Problem::new(NOT_FOUND)
}
