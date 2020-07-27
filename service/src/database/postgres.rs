/// Wrapper around the actual database connection pool.
pub struct Database {}

impl Database {
    /// Create a new connection pool connecting to the provided URL
    ///
    /// # Parameters
    /// - `url` - The URL to connect to
    ///
    /// # Returns
    /// The database wrapper
    pub async fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        let url = url.into();
        tracing::info!(url = ?url, "Connecting to database");
        Database {}
    }
}
