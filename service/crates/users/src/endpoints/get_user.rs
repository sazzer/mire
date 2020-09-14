use super::model::UserModel;
use crate::service::UsersService;
use crate::UserId;
use actix_web::web::{Data, Path};
use mire_authorization::Authenticator;
use mire_problem::{Problem, NOT_FOUND};

/// HTTP Handler for getting the details of a User by ID.
///
/// # Parameters
/// - `users_service` - The Users Service
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/users/:id"),
    skip(users_service)
)]
pub async fn get_user(
    path: Path<UserId>,
    authenticator: Authenticator,
    users_service: Data<UsersService>,
) -> Result<UserModel, Problem> {
    let user_id = &path.0;
    authenticator
        .check()
        .same_principal(&user_id.clone().into())
        .result()?;

    let user = users_service.get_by_id(user_id).await;
    tracing::debug!("Found user: {:?}", user);

    user.map(UserModel::from)
        .ok_or_else(|| Problem::new(NOT_FOUND))
}
