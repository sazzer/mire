use super::{SaveUserError, UsersRepository};
use crate::UserModel;
use chrono::Utc;
use uuid::Uuid;

impl UsersRepository {
    /// Update an existing user with the provided details to describe them.
    ///
    /// # Parameters
    /// - `data` - The details of the user to update
    ///
    /// # Returns
    /// The newly updated user
    ///
    /// # Errors
    /// If the user is unable to be updated for any reason
    pub async fn update(&self, data: UserModel) -> Result<UserModel, SaveUserError> {
        tracing::debug!("Updating user");
        let conn = self.database.checkout().await.unwrap();

        let new_version = Uuid::new_v4();
        let new_updated = Utc::now();
        let authentications = serde_json::to_value(data.data.authentications).unwrap();

        let result = conn.query_opt("UPDATE users
                        SET version = $3, updated = $4, email = $5, display_name = $6, authentications = $7
                        WHERE user_id = $1 AND version = $2
                        RETURNING *", &[
                          &data.identity.id,
                          &data.identity.version,
                          &new_version,
                          &new_updated,
                          &data.data.email,
                          &data.data.display_name,
                          &authentications
                        ])
                        .await
                        .map(|row| row.map(UserModel::from))?;

        if let Some(user) = result {
            tracing::info!(user = ?user, "Updated user");
            Ok(user)
        } else {
            let row = conn
                .query_one(
                    "SELECT COUNT(*) AS c FROM users WHERE user_id = $1",
                    &[&data.identity.id],
                )
                .await?;
            let count: i64 = row.get("c");

            if count == 0 {
                Err(SaveUserError::UnknownUser)
            } else {
                Err(SaveUserError::IncorrectVersion)
            }
        }
    }
}
