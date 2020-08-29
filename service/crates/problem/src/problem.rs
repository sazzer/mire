use actix_http::http::StatusCode;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

/// Trait to represent the type of problem.
pub trait ProblemType<'a>: Debug {
    /// A URI Reference that identifies the problem type.
    fn problem_type(&self) -> &'a str;
}

impl<'a> Display for dyn ProblemType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.problem_type())
    }
}

/// Representation of an RFC-7807 Problem.
#[derive(Debug)]
pub struct Problem<'a> {
    /// The actual error that occurred
    pub error: Box<dyn ProblemType<'a>>,
    /// The HTTP Status code to use
    pub status: StatusCode,
    /// An additional detail message
    pub detail: Option<String>,
    /// An additional instance subtype
    pub instance: Option<String>,
    /// Any extra details
    pub extra: HashMap<String, Value>,
}

impl<'a> Problem<'a> {
    /// Create a new Problem instance
    ///
    /// # Parameters
    /// - `error` - The error code
    /// - `status` - The HTTP Status code
    ///
    /// # Returns
    /// The problem
    pub fn new<T>(error: T, status: StatusCode) -> Self
    where
        T: ProblemType<'a> + 'static,
    {
        Self {
            error: Box::new(error),
            status,
            detail: None,
            instance: None,
            extra: HashMap::new(),
        }
    }

    /// Set the Detail of the Problem instance
    #[allow(dead_code)]
    pub fn with_detail<S>(self, detail: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            detail: Some(detail.into()),
            ..self
        }
    }

    /// Set the Instance of the Problem instance
    #[allow(dead_code)]
    pub fn with_instance<S>(self, instance: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            instance: Some(instance.into()),
            ..self
        }
    }

    /// Set some extra data on the Problem instance
    pub fn with_extra<K, V>(self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        let mut extra = self.extra;
        extra.insert(
            key.into(),
            serde_json::to_value(value).expect("Failed to serialize extra detail"),
        );

        Self { extra, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum ProblemDetails {
        SomeProblem,
    }

    impl<'a> ProblemType<'a> for ProblemDetails {
        fn problem_type(&self) -> &'a str {
            "tag:mire,2020:some/problem"
        }
    }

    #[test]
    fn test_basic_problem() {
        let problem = Problem::new(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST);

        assert_eq!(StatusCode::BAD_REQUEST, problem.status);
        assert_eq!("tag:mire,2020:some/problem", problem.error.problem_type());
        assert_eq!(None, problem.detail);
        assert_eq!(None, problem.instance);
        assert_eq!(0, problem.extra.len());
    }

    #[test]
    fn test_full_problem() {
        let problem = Problem::new(ProblemDetails::SomeProblem, StatusCode::BAD_REQUEST)
            .with_detail("Some Detail")
            .with_instance("Some Instance")
            .with_extra("some_key", "Some Value")
            .with_extra("other_key", 42);

        assert_eq!(StatusCode::BAD_REQUEST, problem.status);
        assert_eq!("tag:mire,2020:some/problem", problem.error.problem_type());
        assert_eq!(Some("Some Detail".to_owned()), problem.detail);
        assert_eq!(Some("Some Instance".to_owned()), problem.instance);
        assert_eq!(2, problem.extra.len());
        assert_eq!(
            Some(&serde_json::to_value("Some Value").unwrap()),
            problem.extra.get(&"some_key".to_owned())
        );
        assert_eq!(
            Some(&serde_json::to_value(42).unwrap()),
            problem.extra.get(&"other_key".to_owned())
        );
    }
}
