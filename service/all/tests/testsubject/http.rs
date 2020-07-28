use super::TestSubject;
use actix_http::Request;
use mire_lib::TestResponse;

impl TestSubject {
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}
