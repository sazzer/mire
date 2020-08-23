use crate::TestSubject;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;

#[actix_rt::test]
async fn test_start_google() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/authentication/google")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::FOUND);
    check!(response.header("cache-control").unwrap() == "no-cache");
}
