use uuid::Uuid;

/// The unique identifier for a Security Context
#[derive(Debug, PartialEq)]
pub struct SecurityContextId(String);

impl Default for SecurityContextId {
    /// Construct a Security Context with a UUID as the value
    fn default() -> Self {
        let id = Uuid::new_v4().to_string();
        Self(id)
    }
}
