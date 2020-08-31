use crate::Problem;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
struct ProblemModel {
    /// The type code for the problem
    pub r#type: String,
    /// The title string for the problem
    pub title: String,
    /// The HTTP Status code to use
    pub status: u16,
    /// An additional detail message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// An additional instance subtype
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Any extra details
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Responder for Problem {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = ProblemModel {
            r#type: self.error.problem_type().to_owned(),
            title: self.error.to_string(),
            status: self.status.as_u16(),
            detail: self.detail,
            instance: self.instance,
            extra: self.extra,
        };

        // Create response and set content type
        ready(Ok(HttpResponse::build(self.status)
            .content_type("application/problem+json")
            .body(serde_json::to_string_pretty(&body).unwrap())))
    }
}
