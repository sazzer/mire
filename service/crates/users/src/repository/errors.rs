use std::borrow::ToOwned;
use tokio_postgres::error::{DbError, SqlState};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SaveUserError {
    /// The email address is already registered
    #[error("The email address is already registered")]
    DuplicateEmail,

    /// An unexpected error occurred
    #[error("An unexpected error occurred")]
    UnexpectedError,
}

impl From<tokio_postgres::Error> for SaveUserError {
    /// Convert a database error into a `SaveUserError`.
    ///
    /// # Parameters
    /// - `e` - The error to convert
    ///
    /// # Returns
    /// The new error code
    fn from(e: tokio_postgres::Error) -> Self {
        if e.code() == Some(&SqlState::UNIQUE_VIOLATION) {
            let db_error: Option<DbError> = e
                .into_source()
                .and_then(|e| e.downcast_ref::<DbError>().cloned());

            db_error
                .and_then(|e| e.constraint().map(ToOwned::to_owned))
                .map(|constraint| {
                    if constraint == "users_email_key" {
                        Self::DuplicateEmail
                    } else {
                        tracing::warn!("Unexpected constraint violation error: {:?}", constraint);
                        Self::UnexpectedError
                    }
                })
                .unwrap_or(Self::UnexpectedError)
        } else {
            tracing::warn!("Unexpected database error: {:?}", e);
            Self::UnexpectedError
        }
    }
}
