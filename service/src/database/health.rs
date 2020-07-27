use super::Database;
use crate::health::Healthchecker;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
impl Healthchecker for Database {
    /// Check the health of the component.
    ///
    /// # Returns
    /// If the component is healthy then returns `Ok(())`.
    /// If the component is unhealthy then returns `Err()` containing the reason why the component is unhealthy.
    async fn check_health(&self) -> Result<(), Box<dyn Error>> {
        let conn = self.pool.get().await?;

        conn.query_one("SELECT 1", &[]).await?;

        Ok(())
    }
}
