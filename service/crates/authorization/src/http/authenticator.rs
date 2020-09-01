use crate::SecurityContext;

/// Means to authenticate an HTTP request
#[derive(Debug)]
pub struct Authenticator(pub(super) Option<SecurityContext>);
