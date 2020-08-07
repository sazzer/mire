use super::{SaveUserError, UsersRepository};
use crate::{UserData, UserId, UserModel};
use chrono::Utc;
use uuid::Uuid;

impl UsersRepository {
    /// Create a new user with the provided details to describe them.
    ///
    /// # Parameters
    /// - `data` - The details of the user to create
    ///
    /// # Returns
    /// The newly created user
    ///
    /// # Errors
    /// If the user is unable to be created for any reason
    pub async fn create(&self, data: UserData) -> Result<UserModel, SaveUserError> {
        tracing::debug!("Creating user");
        let conn = self.database.checkout().await.unwrap();

        let user_id = UserId::default();
        let version = Uuid::new_v4();
        let now = Utc::now();
        let authentications = serde_json::to_value(data.authentications).unwrap();

        let user = conn.query_one("INSERT INTO users(user_id, version, created, updated, email, display_name, authentications)
                        VALUES ($1, $2, $3, $3, $4, $5, $6)
                        RETURNING *", &[
                          &user_id,
                          &version,
                          &now,
                          &data.email,
                          &data.display_name,
                          &authentications
                        ])
                        .await
                        .map(UserModel::from)?;

        tracing::info!(user = ?user, "Created user");
        Ok(user)
    }
}
