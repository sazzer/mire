use mire_database::Database;
use mire_testdatabase::TestDatabase;
use mire_world::{config::WorldConfig, StatsService};
use std::sync::Arc;

pub struct TestWorldService {
    pub test_database: TestDatabase,
    pub stats_service: Arc<StatsService>,
}

impl TestWorldService {
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let test_database = TestDatabase::new();
        let database = Arc::new(Database::new(&test_database.url).await);
        let config = WorldConfig::new(database);

        Self {
            test_database,
            stats_service: config.stats_service,
        }
    }
}
