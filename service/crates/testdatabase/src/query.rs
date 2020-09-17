use super::TestDatabase;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;
use tokio_postgres::{types::ToSql, Row};

impl TestDatabase {
    /// Create a new connection to the database for whatever purpose.
    ///
    /// Note that this actually creates a whole connection pool instead of just one connection.
    /// This is because we can't use the sync `postgres` crate inside of an async runtime, and to use
    /// the async `tokio-postgres` crate comes with a lot of overhead that `deadpool` takes care of for us.
    ///
    /// # Returns
    /// The connection pool.
    async fn connect(&self) -> Pool {
        let pg_config = tokio_postgres::Config::from_str(&self.url).unwrap();

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Clean,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        Pool::new(mgr, 1)
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
        let pool = self.connect().await;
        let conn = pool.get().await.unwrap();

        let rows = conn.query(sql, &binds[..]).await.unwrap();
        callback(rows)
    }

    /// Execute a query against the database and return the number of rows updated
    ///
    /// # Parameters
    /// - `sql` - The SQL to execute
    /// - `binds` - The binds for the SQL, if any
    ///
    /// # Returns
    /// The number of rows updated by the SQL
    pub async fn execute(&self, sql: &str, binds: Vec<&(dyn ToSql + Sync)>) -> u64 {
        let pool = self.connect().await;
        let conn = pool.get().await.unwrap();

        conn.execute(sql, &binds[..]).await.unwrap()
    }
}
