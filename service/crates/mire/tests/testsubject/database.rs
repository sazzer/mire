use super::TestSubject;
use mire_testdata::SeedData;
use tokio_postgres::{types::ToSql, Row};

impl TestSubject {
    pub async fn seed(&self, data: &dyn SeedData) {
        self.database.seed(data).await;
    }

    /// Execute a query against the database and then trigger the provided callback with the results.
    ///
    /// # Parameters
    /// - `sql` - The SQL to execute
    /// - `binds` - The binds for the SQL, if any
    /// - `callback` - The callback to call with the rows returned from the database. May be the empty vec.
    pub async fn query<F>(&self, sql: &str, binds: Vec<&(dyn ToSql + Sync)>, callback: F)
    where
        F: Fn(Vec<Row>),
    {
        self.database.query(sql, binds, callback).await
    }
}
