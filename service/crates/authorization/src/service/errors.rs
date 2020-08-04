/// Errors that can happen when verifying a signed security context
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum VerifyError {
    /// No principal was present in the security context
    #[error("No principal was present in the security context")]
    MissingPrincipal,

    /// The security context has expired
    #[error("The security context has expired")]
    Expired,

    /// The security context was malformed in some way
    #[error("The security context was malformed in some way")]
    Malformed,

    /// An unexpected error occurred
    #[error("An unexpected error occurred")]
    UnexpectedError,
}
