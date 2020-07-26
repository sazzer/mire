mod http;

/// The test wrapper around the service being tested
pub struct TestSubject {
    service: mire_lib::Service,
}

impl TestSubject {
    /// Create a new test subject
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let service = mire_lib::Service::new().await;

        Self { service }
    }
}
