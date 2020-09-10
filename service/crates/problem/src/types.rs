use crate::{FieldValidationType, ProblemType, ProblemTypeStatus};
use actix_http::http::StatusCode;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SimpleProblemType {
    pub problem_type: &'static str,
    pub problem_title: &'static str,
    pub status_code: StatusCode,
}

impl ProblemType for SimpleProblemType {
    /// A URI Reference that identifies the problem type.
    fn problem_type(&self) -> &'static str {
        self.problem_type
    }
}

impl ProblemTypeStatus for SimpleProblemType {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

impl Display for SimpleProblemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.problem_title)
    }
}

pub const NOT_FOUND: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:mire/2020:problems/not_found",
    problem_title: "The requested resource was not found",
    status_code: StatusCode::NOT_FOUND,
};

pub const MISSING_ETAG: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:mire/2020:problems/no_if-match_header",
    problem_title: "The required 'If-Match' header was missing",
    status_code: StatusCode::PRECONDITION_REQUIRED,
};

pub const INCORRECT_VERSION: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:mire/2020:problems/incorrect_if-match_header",
    problem_title: "The required 'If-Match' header was not the correct value",
    status_code: StatusCode::CONFLICT,
};

pub const UNEXPECTED_ERROR: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:mire/2020:problems/internal_server_error",
    problem_title: "An unexpected error occurred",
    status_code: StatusCode::INTERNAL_SERVER_ERROR,
};

#[derive(Debug)]
pub struct SimpleValidationType {
    pub problem_type: &'static str,
    pub problem_title: &'static str,
}

impl FieldValidationType for SimpleValidationType {
    /// A URI Reference that identifies the field validation type.
    fn problem_type(&self) -> &'static str {
        self.problem_type
    }
}

impl Display for SimpleValidationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.problem_title)
    }
}

pub const REQUIRED_FIELD_VALIDATION: SimpleValidationType = SimpleValidationType {
    problem_type: "tag:mire/2020:validation/required_field",
    problem_title: "The field is required but was not present",
};
