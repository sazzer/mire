use crate::{AuthenticationId, AuthenticationProvider, Email, UserId};
use actix_web::{http::header, Error, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use futures::future::{ready, Ready};
use serde::Serialize;
use uuid::Uuid;

/// API Model to represent the authentication details of a user
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationModel {
    /// The identity of the provider these details relate to
    pub authentication_provider: AuthenticationProvider,
    /// The ID of the user at this provider
    pub authentication_id: AuthenticationId,
    /// The display name of these details at the provider
    pub display_name: String,
}

/// API Model to represent a user
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
    /// The ID of the user
    pub id: UserId,
    /// The version of the user
    #[serde(skip)]
    pub version: Uuid,
    /// When the resource was created
    pub created: DateTime<Utc>,
    /// When the resource was last updated
    pub updated: DateTime<Utc>,
    /// The display name of the user
    pub display_name: String,
    /// The email address of the user
    pub email: Email,
    /// The set of authentication details of this user
    pub authentications: Vec<AuthenticationModel>,
}

impl From<crate::Authentication> for AuthenticationModel {
    fn from(authentication: crate::Authentication) -> Self {
        Self {
            authentication_id: authentication.authentication_id,
            authentication_provider: authentication.authentication_provider,
            display_name: authentication.display_name,
        }
    }
}

impl From<crate::UserModel> for UserModel {
    fn from(user: crate::UserModel) -> Self {
        Self {
            id: user.identity.id,
            version: user.identity.version,
            created: user.identity.created,
            updated: user.identity.updated,
            display_name: user.data.display_name,
            email: user.data.email,
            authentications: user
                .data
                .authentications
                .into_iter()
                .map(AuthenticationModel::from)
                .collect(),
        }
    }
}

impl From<&UserModel> for HttpResponse {
    fn from(user: &UserModel) -> Self {
        Self::Ok()
            .set(header::ETag(header::EntityTag::new(
                false,
                user.version.to_string(),
            )))
            .set(header::CacheControl(vec![
                header::CacheDirective::Private,
                header::CacheDirective::MaxAge(3600),
            ]))
            .json(user)
    }
}

impl From<UserModel> for HttpResponse {
    fn from(user: UserModel) -> Self {
        Self::from(&user)
    }
}

impl Responder for UserModel {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // Create response and set content type
        ready(Ok(self.into()))
    }
}
