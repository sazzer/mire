use crate::TestSubject;
use actix_http::http::StatusCode;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use uuid::Uuid;

#[actix_rt::test]
async fn test_get_unknown_stat() {
    let test_subject = TestSubject::new().await;

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/stats/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
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
async fn test_get_known_stat() {
    let test_subject = TestSubject::new().await;

    test_subject
        .seed(&mire_testdata::SeedStat {
            stat_id: Uuid::parse_str("b077bffd-7d03-4c85-ad33-d0b1e8a7daa5").unwrap(),
            name: "Strength".to_owned(),
            description: "How strong you are".to_owned(),
            default_value: 10,
        })
        .await;

    let response = test_subject
        .inject(
            TestRequest::get()
                .uri("/stats/b077bffd-7d03-4c85-ad33-d0b1e8a7daa5")
                .to_request(),
        )
        .await;

    check!(response.status == StatusCode::OK);
    check!(response.header("content-type").unwrap() == "application/json");
    check!(response.header("cache-control").unwrap() == "private, max-age=3600");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "id": "b077bffd-7d03-4c85-ad33-d0b1e8a7daa5",
      "name": "Strength",
      "description": "How strong you are",
      "defaultValue": 10
    }
    "###);
}
