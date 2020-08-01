/// The unique identifier for a Principal that can be authorized to perform actions
#[derive(Debug, PartialEq)]
pub enum PrincipalId {
    /// The Principal is a User in the system.
    /// In this case, the value exactly equates to a User ID from the users domain.
    User(String),
}
