mod database;
mod http;
mod postgres;

/// The test wrapper around the service being tested
pub struct TestSubject {
    service: mire_lib::Service,
    #[allow(dead_code)]
    database: database::TestDatabase,
}

impl TestSubject {
    /// Create a new test subject
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let database = database::TestDatabase::new();

        let service = mire_lib::Service::new().await;

        Self { service, database }
    }
}
