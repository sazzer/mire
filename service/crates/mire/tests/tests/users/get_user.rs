use crate::TestSubject;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use chrono::{TimeZone, Utc};
use insta::assert_json_snapshot;
use uuid::Uuid;

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

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
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

#[actix_rt::test]
async fn test_get_known_user() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                created: Utc.ymd(2020, 8, 1).and_hms(11, 11, 12),
                updated: Utc.ymd(2020, 9, 2).and_hms(12, 34, 56),
                email: "testing@example.com".to_owned(),
                display_name: "Testing".to_owned(),
                ..mire_testdata::SeedUser::default()
            }
            .with_authentication("google", "googleUserId", "testuser@example.com"),
        )
        .await;

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    check!(response.header("cache-control").unwrap() == "private, max-age=3600");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "id": "b077bffd-7d03-4c85-ad33-d0b1e8a7daa5",
      "created": "2020-08-01T11:11:12Z",
      "updated": "2020-09-02T12:34:56Z",
      "displayName": "Testing",
      "email": "testing@example.com",
      "authentications": [
        {
          "authenticationProvider": "google",
          "authenticationId": "googleUserId",
          "displayName": "testuser@example.com"
        }
      ]
    }
    "###);
}
