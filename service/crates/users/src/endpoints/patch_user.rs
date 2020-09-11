use crate::{
    endpoints::model::UserModel, DisplayName, Email, UpdateUserError, UserId, UsersService,
};
use actix_http::http::StatusCode;
use actix_web::{
    http::header,
    web::{Data, HttpRequest, Json, Path},
};
use mire_authorization::Authenticator;
use mire_problem::{
    Problem, SimpleProblemType, ValidationErrorBuilder, INCORRECT_VERSION, MISSING_ETAG, NOT_FOUND,
    REQUIRED_FIELD_VALIDATION, UNEXPECTED_ERROR,
};
use serde::Deserialize;
use std::str::FromStr;

/// The shape of the body for the patch request
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchBody {
    display_name: Option<String>,
    email: Option<String>,
}

pub const DUPLICATE_EMAIL: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:mire/2020:users/problems/duplicate_email",
    problem_title: "The email address is already registered to another user",
    status_code: StatusCode::UNPROCESSABLE_ENTITY,
};

/// HTTP Handler for updating the details of a User.
///
/// # Parameters
/// - `users_service` - The Users Service
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "PATCH", http_path = "/users/:id"),
    skip(users_service, req)
)]
pub async fn patch_user(
    path: Path<(UserId,)>,
    body: Json<PatchBody>,
    req: HttpRequest,
    authenticator: Authenticator,
    users_service: Data<UsersService>,
) -> Result<UserModel, Problem> {
    let user_id = &path.0;
    authenticator
        .check()
        .same_principal(&user_id.clone().into())
        .result()?;

    let version = req
        .headers()
        .get(header::IF_MATCH)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| header::EntityTag::from_str(v).ok())
        .filter(|etag| !etag.weak)
        .ok_or_else(|| Problem::new(MISSING_ETAG))?;

    let version =
        uuid::Uuid::from_str(version.tag()).map_err(|_| Problem::new(INCORRECT_VERSION))?;

    let email = body.email.clone().map(|e| Email::from_str(&e)).transpose();
    let display_name = body
        .display_name
        .clone()
        .map(|e| DisplayName::from_str(&e))
        .transpose();

    if let (Ok(email), Ok(display_name)) = (&email, &display_name) {
        let user = users_service
            .update(user_id, |user| {
                user.identity.version = version;
                if let Some(email) = email {
                    user.data.email = email.clone();
                }
                if let Some(display_name) = display_name {
                    user.data.display_name = display_name.clone();
                }
            })
            .await;
        tracing::debug!("Updated user: {:?}", user);

        user.map(UserModel::from).map_err(|e| match e {
            UpdateUserError::IncorrectVersion => Problem::new(INCORRECT_VERSION),
            UpdateUserError::UnknownUser => Problem::new(NOT_FOUND),
            UpdateUserError::DuplicateEmail => Problem::new(DUPLICATE_EMAIL),
            UpdateUserError::UnexpectedError => Problem::new(UNEXPECTED_ERROR),
        })
    } else {
        let mut builder = ValidationErrorBuilder::default();

        if email.is_err() {
            builder.with_field("email", &REQUIRED_FIELD_VALIDATION);
        }

        if display_name.is_err() {
            builder.with_field("displayName", &REQUIRED_FIELD_VALIDATION);
        }

        Err(builder.build())
    }
}
