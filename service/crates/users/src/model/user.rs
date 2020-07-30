use super::{Authentication, Email, UserId};
use mire_model::Model;

/// Representation of the data that makes up a user
pub struct UserData {
    /// The display name of the user
    pub display_name: String,
    /// The email address of the user
    pub email: Email,
    /// The set of authentication details of this user
    pub authentications: Vec<Authentication>,
}

/// Model type for a persisted user
pub type UserModel = Model<UserId, UserData>;
