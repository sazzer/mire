use super::TestDatabase;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use mire_testdata::SeedData;
use std::str::FromStr;
use tokio_postgres::config::Config;

impl TestDatabase {
    /// Seed the provided data into the database
    pub async fn seed(&self, data: &dyn SeedData) {
        tracing::debug!(data = ?data, "Seeding database");

        let config = Config::from_str(&self.url).unwrap();
        let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);
        let pool = Pool::builder()
            .build(pg_mgr)
            .await
            .expect("Failed to create database connection pool");

        let conn = pool.get().await.unwrap();

        let rows = conn.execute(data.sql(), &data.binds()[..]).await.unwrap();
        tracing::debug!(rows = rows, "Seeded database");
    }
}
