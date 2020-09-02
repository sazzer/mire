use crate::TestSubject;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn test_get_user_unauthenticated() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::FORBIDDEN);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    check!(response.header("cache-control") == None);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:mire/2020:problems/forbidden",
      "title": "The request was not permitted to perform this action",
      "status": 403
    }
    "###);
}

#[actix_rt::test]
async fn test_get_user_invalid_token() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("Authorization", "Bearer ThisIsInvalid")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::UNAUTHORIZED);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    check!(response.header("cache-control") == None);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:mire/2020:problems/unauthenticated",
      "title": "The request was not correctly authenticated",
      "status": 401
    }
    "###);
}

#[actix_rt::test]
async fn test_get_unknown_user() {
    let test_subject = TestSubject::new().await;

    let token = test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5");

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("Authorization", format!("Bearer {}", token))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::NOT_FOUND);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    check!(response.header("cache-control") == None);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:mire/2020:problems/not_found",
      "title": "The requested resource was not found",
      "status": 404
    }
    "###);
}
