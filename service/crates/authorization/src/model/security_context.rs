use super::PrincipalId;
use super::SecurityContextId;
use chrono::{DateTime, Utc};

/// Security Context representing the authorization that a given request has
#[derive(Debug)]
pub struct SecurityContext {
    /// The unique ID of this security context
    pub id: SecurityContextId,
    /// The principal that this security context is for
    pub principal_id: PrincipalId,
    /// The earliest time that this security context is valid
    pub not_valid_before: DateTime<Utc>,
    /// The latest time that this security context is valid
    pub not_valid_after: DateTime<Utc>,
}
