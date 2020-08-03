#[derive(thiserror::Error, Debug, PartialEq)]
pub enum VerifyError {
    #[error("No principal was present in the security context")]
    MissingPrincipal,

    #[error("The security context has expired")]
    Expired,

    #[error("The security context was malformed in some way")]
    Malformed,

    #[error("An unexpected error occurred")]
    UnexpectedError,
}
