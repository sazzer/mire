mod create;
mod get_by_authentication;
mod get_by_id;

use super::repository::UsersRepository;
pub use create::CreateUserError;

/// Service for interacting with user resources
pub struct UsersService {
    /// The repository for accessing user data
    #[allow(dead_code)]
    pub(super) repository: UsersRepository,
}
