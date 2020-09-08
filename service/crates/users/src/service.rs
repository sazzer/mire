mod create;
mod get_by_authentication;
mod get_by_id;
mod update;

use super::repository::UsersRepository;
pub use create::CreateUserError;
pub use update::*;

/// Service for interacting with user resources
#[derive(Clone)]
pub struct UsersService {
    /// The repository for accessing user data
    #[allow(dead_code)]
    pub(super) repository: UsersRepository,
}
