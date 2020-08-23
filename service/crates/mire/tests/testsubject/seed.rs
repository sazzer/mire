use super::TestSubject;
use mire_testdata::SeedData;

impl TestSubject {
    pub async fn seed(&self, data: &dyn SeedData) {
        self.database.seed(data).await;
    }
}
