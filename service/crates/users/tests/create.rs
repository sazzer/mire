mod service;

use assert2::{assert, check};

#[actix_rt::test]
async fn create_success() {
    let service = service::TestUsersService::new().await;

    let result = service
        .users_service
        .create(mire_users::UserData {
            email: "testuser@example.com".parse().unwrap(),
            display_name: "Test User".to_owned(),
            authentications: vec![mire_users::Authentication {
                authentication_provider: "google".parse().unwrap(),
                authentication_id: "123456".parse().unwrap(),
                display_name: "test@example.com".to_owned(),
            }],
        })
        .await;

    assert!(result.is_ok());
    let user = result.unwrap();
    check!(user.data.email == "testuser@example.com".parse().unwrap());
    check!(user.data.display_name == "Test User");
    check!(user.data.authentications.len() == 1);
    check!(user.data.authentications[0].authentication_provider == "google".parse().unwrap());
    check!(user.data.authentications[0].authentication_id == "123456".parse().unwrap());
    check!(user.data.authentications[0].display_name == "test@example.com");
}

#[actix_rt::test]
async fn create_refetch() {
    let service = service::TestUsersService::new().await;

    let result = service
        .users_service
        .create(mire_users::UserData {
            email: "testuser@example.com".parse().unwrap(),
            display_name: "Test User".to_owned(),
            authentications: vec![mire_users::Authentication {
                authentication_provider: "google".parse().unwrap(),
                authentication_id: "123456".parse().unwrap(),
                display_name: "test@example.com".to_owned(),
            }],
        })
        .await;

    assert!(result.is_ok());
    let created_user = result.unwrap();

    let user = service
        .users_service
        .get_by_id(&created_user.identity.id)
        .await;
    assert!(user.is_some());

    let fetched_user = user.unwrap();
    check!(fetched_user == created_user);
}

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
                display_name: "test@example.com".to_owned(),
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

#[actix_rt::test]
async fn create_duplicate_email() {
    let seeded_user = mire_testdata::SeedUser {
        email: "testuser@example.com".to_owned(),
        ..mire_testdata::SeedUser::default()
    };

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
                display_name: "test@example.com".to_owned(),
            }],
        })
        .await;

    assert!(result.is_err());
    check!(result.unwrap_err() == mire_users::CreateUserError::DuplicateEmail);
}
