use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;
use tokio_postgres::config::Config;

/// Wrapper around the actual database connection pool.
pub struct Database {
    #[allow(dead_code)]
    pub(super) pool: Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,
}

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

        let config = Config::from_str(&url).unwrap();
        let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);
        let pool = Pool::builder()
            .build(pg_mgr)
            .await
            .expect("Failed to create database connection pool");

        Database { pool }
    }

    /// Check out a connection from the database pool in order to make queries
    ///
    /// # Returns
    /// The connection to use
    ///
    /// # Errors
    /// If the pool is unable to return a viable connection
    pub async fn checkout(
        &self,
    ) -> Result<
        bb8::PooledConnection<'_, PostgresConnectionManager<tokio_postgres::NoTls>>,
        bb8::RunError<tokio_postgres::Error>,
    > {
        let conn = self.pool.get().await?;

        Ok(conn)
    }
}
