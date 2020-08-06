use super::{Authentication, Email, UserId};
use mire_model::Identity;

/// Representation of the data that makes up a user
#[derive(Debug)]
pub struct UserData {
    /// The display name of the user
    pub display_name: String,
    /// The email address of the user
    pub email: Email,
    /// The set of authentication details of this user
    pub authentications: Vec<Authentication>,
}

/// Model type for a persisted user
#[derive(Debug)]
pub struct UserModel {
    /// The identity of the resource
    pub identity: Identity<UserId>,
    /// The data of the resource
    pub data: UserData,
}
