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

#[actix_rt::test]
async fn test_complete_google_authentication_failed() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/authentication/google/complete?error=access_denied")
                .to_request(),
        )
        .await;
    check!(response.status == StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn test_complete_google_invalid_code() {
    let test_subject = TestSubject::new().await;

    let m = mockito::mock("POST", "/authentication/google/oauth2/v4/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body("code=invalid&client_id=GoogleClientId&client_secret=GoogleClientSecret&redirect_uri=http%3A%2F%2Flocalhost%2Fauthentication%2Fgoogle%2Fredirect&grant_type=authorization_code")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "invalid_request"}"#)
        .create();

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/authentication/google/complete?code=invalid")
                .to_request(),
        )
        .await;
    check!(response.status == StatusCode::BAD_REQUEST);

    m.assert();
}

#[actix_rt::test]
async fn test_complete_google_missing_id_token() {
    let test_subject = TestSubject::new().await;

    let m = mockito::mock("POST", "/authentication/google/oauth2/v4/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body("code=valid&client_id=GoogleClientId&client_secret=GoogleClientSecret&redirect_uri=http%3A%2F%2Flocalhost%2Fauthentication%2Fgoogle%2Fredirect&grant_type=authorization_code")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
          "access_token":"2YotnFZFEjr1zCsicMWpAA",
          "token_type":"Bearer",
          "expires_in":3600
        }"#)
        .create();

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/authentication/google/complete?code=valid")
                .to_request(),
        )
        .await;
    check!(response.status == StatusCode::BAD_REQUEST);

    m.assert();
}

#[actix_rt::test]
async fn test_complete_google_invalid_id_token() {
    let test_subject = TestSubject::new().await;

    let m = mockito::mock("POST", "/authentication/google/oauth2/v4/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body("code=valid&client_id=GoogleClientId&client_secret=GoogleClientSecret&redirect_uri=http%3A%2F%2Flocalhost%2Fauthentication%2Fgoogle%2Fredirect&grant_type=authorization_code")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
          "access_token":"2YotnFZFEjr1zCsicMWpAA",
          "token_type":"Bearer",
          "expires_in":3600,
          "id_token": "invalid"
        }"#)
        .create();

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/authentication/google/complete?code=valid")
                .to_request(),
        )
        .await;
    check!(response.status == StatusCode::BAD_REQUEST);

    m.assert();
}

#[actix_rt::test]
async fn test_complete_google_valid_id_token_unknown_user() {
    let test_subject = TestSubject::new().await;

    let m = mockito::mock("POST", "/authentication/google/oauth2/v4/token")
        .match_header("content-type", "application/x-www-form-urlencoded")
        .match_body("code=valid&client_id=GoogleClientId&client_secret=GoogleClientSecret&redirect_uri=http%3A%2F%2Flocalhost%2Fauthentication%2Fgoogle%2Fredirect&grant_type=authorization_code")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
          "access_token":"2YotnFZFEjr1zCsicMWpAA",
          "token_type":"Bearer",
          "expires_in":3600,
          "id_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJnb29nbGVVc2VySWQiLCJlbWFpbCI6InRlc3R1c2VyQGV4YW1wbGUuY29tIiwibmFtZSI6IlRlc3QgVXNlciJ9.j1oy1X7lHXoZFFHPwJBNIfPNdYQ-MO7LxdNcIh51LAE"
        }"#)
        .create();

    // The above ID Token represents:
    // * User ID: googleUserId
    // * Email Address: testuser@example.com
    // * Name: Test User

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/authentication/google/complete?code=valid")
                .to_request(),
        )
        .await;
    check!(response.status == StatusCode::OK);

    m.assert();

    // TODO: Assert the database record
}
