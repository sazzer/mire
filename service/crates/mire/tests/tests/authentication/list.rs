use crate::TestSubject;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn test_list_providers() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(TestRequest::get().uri("/authentication").to_request())
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    check!(response.header("cache-control").unwrap() == "public, max-age=3600");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    [
      "google"
    ]
    "###);
}
