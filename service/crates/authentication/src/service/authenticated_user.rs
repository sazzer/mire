use mire_users::{AuthenticationId, DisplayName, Email};

/// The details of the user that authenticated against the external provider
pub struct AuthenticatedUser {
    /// The ID of the user with the provider
    pub provider_id: AuthenticationId,
    /// The display name of the user with the provider
    pub provider_display_name: String,
    /// The display name of the user
    pub user_display_name: DisplayName,
    /// The email address of the user
    pub email: Email,
}
