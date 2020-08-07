mod service;

use assert2::{assert, check};

#[actix_rt::test]
async fn create_duplicate_authentication() {
    let seeded_user = mire_testdata::SeedUser::default().with_authentication(
        "google",
        "123456",
        "test@example.com",
    );

    let service = service::TestUsersService::new().await;
    service.test_database.seed(&seeded_user).await;

    let result = service
        .users_service
        .create(mire_users::UserData {
            email: "testuser@example.com".parse().unwrap(),
            display_name: "Test User".to_owned(),
            authentications: vec![mire_users::Authentication {
                authentication_provider: "google".parse().unwrap(),
                authentication_id: "123456".parse().unwrap(),
                display_name: "Test User".to_owned(),
            }],
        })
        .await;

    assert!(result.is_err());
    check!(
        result.unwrap_err()
            == mire_users::CreateUserError::DuplicateAuthentication(
                "google".parse().unwrap(),
                "123456".parse().unwrap()
            )
    );
}
