use crate::TestSubject;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn test_healthcheck() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(TestRequest::get().uri("/health").to_request())
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "healthy": true,
      "components": {
        "db": {
          "healthy": true
        }
      }
    }
    "###);
}
