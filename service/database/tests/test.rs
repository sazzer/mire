use mire_database::{migrate::migrate, Database};
use mire_testdatabase::TestDatabase;

#[actix_rt::test]
async fn test_migrate_database() {
    let container = TestDatabase::new();

    let database = Database::new(container.url).await;
    migrate(&database).await;
}
