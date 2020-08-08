mod http;
use mire_testdatabase::TestDatabase;

/// The test wrapper around the service being tested
pub struct TestSubject {
    service: mire_lib::Service,
    #[allow(dead_code)]
    database: TestDatabase,
}

impl TestSubject {
    /// Create a new test subject
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let database = TestDatabase::new();

        let config = mire_lib::Config {
            database_url: database.url.clone(),
            ..mire_lib::Config::default()
        };

        let service = mire_lib::Service::new(config).await;

        Self { service, database }
    }
}
