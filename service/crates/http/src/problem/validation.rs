use super::{Problem, SimpleProblemType};
use actix_http::http::StatusCode;
use serde::Serialize;
use std::fmt::{Debug, Display};

pub const VALIDATION_ERROR: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:mire/2020:problems/validation_error",
    problem_title: "A validation error occurred",
    status_code: StatusCode::UNPROCESSABLE_ENTITY,
};

/// Trait to represent the type of field validation error.
pub trait FieldValidationType: Debug + Display {
    /// A URI Reference that identifies the field validation type.
    fn problem_type(&self) -> &'static str;
}

/// Details of a validation error on a field
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldValidation {
    /// The name of the field in error
    field_name: String,
    /// The actual error
    r#type: String,
    /// The human readable description of the error
    title: String,
}

/// Builder for constructing a validation error problem
#[derive(Debug, Default)]
pub struct ValidationErrorBuilder {
    /// The fields that are in error
    fields: Vec<FieldValidation>,
}

impl ValidationErrorBuilder {
    /// Record a new problem with a field
    ///
    /// # Parameters
    /// - `field_name` - The name of the field in error
    /// - `problem` - The problem with the field
    pub fn with_field<S, F>(&mut self, field_name: S, problem: &F) -> &mut Self
    where
        S: Into<String>,
        F: FieldValidationType,
    {
        let r#type = problem.problem_type().to_owned();
        let title = problem.to_string();

        self.fields.push(FieldValidation {
            field_name: field_name.into(),
            r#type,
            title,
        });

        self
    }

    /// Build a problem from the builder
    #[must_use]
    pub fn build(self) -> Problem {
        Problem::new(VALIDATION_ERROR).with_extra("fields", self.fields)
    }
}
