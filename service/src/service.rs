/// The actual service layer
pub struct Service {}

impl Service {
    /// Create a new instance of the service layer
    pub async fn new() -> Service {
        log::info!("Building service");
        Service {}
    }

    /// Start the service running
    pub async fn start(self) {
        log::info!("Starting service");
    }
}
