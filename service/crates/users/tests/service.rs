use mire_database::Database;
use mire_testdatabase::TestDatabase;
use mire_users::{config::UsersConfig, UsersService};
use std::sync::Arc;

pub struct TestUsersService {
    pub test_database: TestDatabase,
    pub users_service: Arc<UsersService>,
}

impl TestUsersService {
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let test_database = TestDatabase::new();
        let database = Arc::new(Database::new(&test_database.url).await);
        let users_config = UsersConfig::new(database);

        Self {
            test_database,
            users_service: users_config.service,
        }
    }
}
