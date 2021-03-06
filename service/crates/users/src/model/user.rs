use super::{Authentication, DisplayName, Email, UserId};
use mire_model::Identity;

/// Representation of the data that makes up a user
#[derive(Debug, PartialEq)]
pub struct UserData {
    /// The display name of the user
    pub display_name: DisplayName,
    /// The email address of the user
    pub email: Email,
    /// The set of authentication details of this user
    pub authentications: Vec<Authentication>,
}

/// Model type for a persisted user
#[derive(Debug, PartialEq)]
pub struct UserModel {
    /// The identity of the resource
    pub identity: Identity<UserId>,
    /// The data of the resource
    pub data: UserData,
}
