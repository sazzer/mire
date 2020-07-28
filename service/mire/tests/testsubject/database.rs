use super::postgres::PostgresContainer;
use lazy_static::lazy_static;
use testcontainers::{clients::Cli, Container, Docker};

lazy_static! {
    static ref DOCKER: Cli = Cli::default();
}

/// Wrapper around a Docker container that runs Postgres for our tests
pub struct TestDatabase {
    #[allow(dead_code)]
    node: Container<'static, Cli, PostgresContainer>,
    pub host: String,
    pub port: u16,
    pub url: String,
}

impl Default for TestDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl TestDatabase {
    #[must_use]
    pub fn new() -> Self {
        tracing::info!("Starting Postgres database");
        let node = DOCKER.run(PostgresContainer::default());

        let host = "localhost".to_owned();
        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres@{}:{}", host, port);
        tracing::info!(url = ?url, "Running postgres");

        TestDatabase {
            node,
            host,
            port,
            url,
        }
    }
}
