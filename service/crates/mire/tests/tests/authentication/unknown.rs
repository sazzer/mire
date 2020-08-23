use crate::TestSubject;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;

#[actix_rt::test]
async fn test_start_unknown() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/authentication/unknown")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::NOT_FOUND);
    check!(response.header("cache-control").unwrap() == "public, max-age=3600");
}
