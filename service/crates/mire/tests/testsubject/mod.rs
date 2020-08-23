mod http;
mod seed;
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
            google_config: Some(mire_authentication::google::Config {
                client_id: "GoogleClientId".to_owned(),
                client_secret: "GoogleClientSecret".to_owned(),
                redirect_uri: "http://localhost/authentication/google/redirect".to_owned(),
                auth_uri: None,
                token_uri: Some(format!(
                    "{}/authentication/google/oauth2/v4/token",
                    mockito::server_url()
                )),
            }),
        };

        let service = mire_lib::Service::new(config).await;

        Self { service, database }
    }
}
