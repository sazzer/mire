use mire_database::Database;
use mire_testdatabase::TestDatabase;

#[actix_rt::test]
async fn test_migrate_database() {
    let container = TestDatabase::new();

    let _ = Database::new(container.url).await;
}
