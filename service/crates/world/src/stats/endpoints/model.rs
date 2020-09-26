use crate::StatId;
use actix_web::{http::header, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

/// API Model to represent a Stat Score
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatModel {
    /// The ID of the Stat Score
    pub id: StatId,
    /// The name of the Stat Score
    pub name: String,
    /// The description of the Stat Score
    pub description: String,
    /// The default value of the Stat Score
    pub default_value: u32,
}

impl From<crate::Stat> for StatModel {
    fn from(stat: crate::Stat) -> Self {
        Self {
            id: stat.id,
            name: stat.name,
            description: stat.description,
            default_value: stat.default_value,
        }
    }
}

impl From<&StatModel> for HttpResponse {
    fn from(stat: &StatModel) -> Self {
        Self::Ok()
            .set(header::CacheControl(vec![
                header::CacheDirective::Private,
                header::CacheDirective::MaxAge(3600),
            ]))
            .json(stat)
    }
}

impl From<StatModel> for HttpResponse {
    fn from(stat: StatModel) -> Self {
        Self::from(&stat)
    }
}

impl Responder for StatModel {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // Create response and set content type
        ready(Ok(self.into()))
    }
}
