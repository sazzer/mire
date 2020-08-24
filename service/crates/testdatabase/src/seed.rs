use super::TestDatabase;
use mire_testdata::SeedData;

impl TestDatabase {
    /// Seed the provided data into the database
    pub async fn seed(&self, data: &dyn SeedData) {
        tracing::debug!(data = ?data, "Seeding database");

        let rows = self.execute(data.sql(), data.binds()).await;

        tracing::debug!(rows = rows, "Seeded database");
    }
}
