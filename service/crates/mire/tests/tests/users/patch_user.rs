use crate::TestSubject;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use chrono::{TimeZone, Utc};
use insta::assert_json_snapshot;
use serde_json::json;
use uuid::Uuid;

#[actix_rt::test]
async fn test_patch_user_unauthenticated() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .set_json(&json!({}))
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
async fn test_patch_known_user_no_etag() {
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
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::PRECONDITION_REQUIRED);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:mire/2020:problems/no_if-match_header",
      "title": "The required 'If-Match' header was missing",
      "status": 428
    }
    "###);
}

#[actix_rt::test]
async fn test_patch_known_user_invalid_etag() {
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
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "WithoutQuotes")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::PRECONDITION_REQUIRED);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:mire/2020:problems/no_if-match_header",
      "title": "The required 'If-Match' header was missing",
      "status": 428
    }
    "###);
}

#[actix_rt::test]
async fn test_patch_known_user_weak_etag() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                version: Uuid::parse_str("020d8250-f190-42c9-b966-fc3e991115e3").unwrap(),
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
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "W/\"020d8250-f190-42c9-b966-fc3e991115e3\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::PRECONDITION_REQUIRED);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:mire/2020:problems/no_if-match_header",
      "title": "The required 'If-Match' header was missing",
      "status": 428
    }
    "###);
}

#[actix_rt::test]
async fn test_patch_known_user_non_uuid_etag() {
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
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "\"NonUuid\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::CONFLICT);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:mire/2020:problems/incorrect_if-match_header",
      "title": "The required 'If-Match' header was not the correct value",
      "status": 409
    }
    "###);
}

#[actix_rt::test]
async fn test_patch_known_user_wrong_etag() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                version: Uuid::parse_str("020d8250-f190-42c9-b966-fc3e991115e3").unwrap(),
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
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "\"020d8250-f190-42c9-b966-fc3e991115e2\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::CONFLICT);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:mire/2020:problems/incorrect_if-match_header",
      "title": "The required 'If-Match' header was not the correct value",
      "status": 409
    }
    "###);
}

#[actix_rt::test]
async fn test_patch_known_user_no_changes() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                version: Uuid::parse_str("020d8250-f190-42c9-b966-fc3e991115e3").unwrap(),
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
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "\"020d8250-f190-42c9-b966-fc3e991115e3\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    assert_json_snapshot!(response.to_json().unwrap(), {
      ".updated" => "[updated-date]"
    }, @r###"
    {
      "id": "b077bffd-7d03-4c85-ad33-d0b1e8a7daa5",
      "created": "2020-08-01T11:11:12Z",
      "updated": "[updated-date]",
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

#[actix_rt::test]
async fn test_patch_known_user_all_changes() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                version: Uuid::parse_str("020d8250-f190-42c9-b966-fc3e991115e3").unwrap(),
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
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "\"020d8250-f190-42c9-b966-fc3e991115e3\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({
                  "displayName": "New Name",
                  "email": "new@example.com"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    assert_json_snapshot!(response.to_json().unwrap(), {
      ".updated" => "[updated-date]"
    }, @r###"
    {
      "id": "b077bffd-7d03-4c85-ad33-d0b1e8a7daa5",
      "created": "2020-08-01T11:11:12Z",
      "updated": "[updated-date]",
      "displayName": "New Name",
      "email": "new@example.com",
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

#[actix_rt::test]
async fn test_patch_known_user_duplicate_email() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                version: Uuid::parse_str("020d8250-f190-42c9-b966-fc3e991115e3").unwrap(),
                email: "testing@example.com".to_owned(),
                ..mire_testdata::SeedUser::default()
            }
            .with_authentication("google", "googleUserId", "testuser@example.com"),
        )
        .await;

    test_subject
        .seed(&mire_testdata::SeedUser {
            email: "new@example.com".to_owned(),
            ..mire_testdata::SeedUser::default()
        })
        .await;

    let response = test_subject
        .inject(
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "\"020d8250-f190-42c9-b966-fc3e991115e3\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({
                  "email": "new@example.com"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::UNPROCESSABLE_ENTITY);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), {
      ".updated" => "[updated-date]"
    }, @r###"
    {
      "type": "tag:mire/2020:users/problems/duplicate_email",
      "title": "The email address is already registered to another user",
      "status": 422
    }
    "###);
}

#[actix_rt::test]
async fn test_patch_known_user_blank_email() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                version: Uuid::parse_str("020d8250-f190-42c9-b966-fc3e991115e3").unwrap(),
                ..mire_testdata::SeedUser::default()
            }
            .with_authentication("google", "googleUserId", "testuser@example.com"),
        )
        .await;

    let response = test_subject
        .inject(
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "\"020d8250-f190-42c9-b966-fc3e991115e3\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({
                  "email": ""
                }))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::UNPROCESSABLE_ENTITY);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), {
      ".updated" => "[updated-date]"
    }, @r###"
    {
      "type": "tag:mire/2020:problems/validation_error",
      "title": "A validation error occurred",
      "status": 422,
      "fields": [
        {
          "fieldName": "email",
          "type": "tag:mire/2020:validation/required_field",
          "title": "The field is required but was not present"
        }
      ]
    }
    "###);
}

#[actix_rt::test]
async fn test_patch_known_user_blank_display_name() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                version: Uuid::parse_str("020d8250-f190-42c9-b966-fc3e991115e3").unwrap(),
                ..mire_testdata::SeedUser::default()
            }
            .with_authentication("google", "googleUserId", "testuser@example.com"),
        )
        .await;

    let response = test_subject
        .inject(
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "\"020d8250-f190-42c9-b966-fc3e991115e3\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({
                  "displayName": ""
                }))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::UNPROCESSABLE_ENTITY);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), {
      ".updated" => "[updated-date]"
    }, @r###"
    {
      "type": "tag:mire/2020:problems/validation_error",
      "title": "A validation error occurred",
      "status": 422,
      "fields": [
        {
          "fieldName": "displayName",
          "type": "tag:mire/2020:validation/required_field",
          "title": "The field is required but was not present"
        }
      ]
    }
    "###);
}

#[actix_rt::test]
async fn test_patch_known_user_blank_display_name_email() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(
            &mire_testdata::SeedUser {
                user_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
                version: Uuid::parse_str("020d8250-f190-42c9-b966-fc3e991115e3").unwrap(),
                ..mire_testdata::SeedUser::default()
            }
            .with_authentication("google", "googleUserId", "testuser@example.com"),
        )
        .await;

    let response = test_subject
        .inject(
            TestRequest::patch()
                .uri("/users/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .header("if-match", "\"020d8250-f190-42c9-b966-fc3e991115e3\"")
                .set(test_subject.generate_access_token("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5"))
                .set_json(&json!({
                  "displayName": "",
                  "email": ""
                }))
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::UNPROCESSABLE_ENTITY);
    check!(response.header("content-type").unwrap() == "application/problem+json");
    assert_json_snapshot!(response.to_json().unwrap(), {
      ".updated" => "[updated-date]"
    }, @r###"
    {
      "type": "tag:mire/2020:problems/validation_error",
      "title": "A validation error occurred",
      "status": 422,
      "fields": [
        {
          "fieldName": "email",
          "type": "tag:mire/2020:validation/required_field",
          "title": "The field is required but was not present"
        },
        {
          "fieldName": "displayName",
          "type": "tag:mire/2020:validation/required_field",
          "title": "The field is required but was not present"
        }
      ]
    }
    "###);
}
