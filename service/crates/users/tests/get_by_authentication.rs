mod service;

use assert2::{assert, check};

#[actix_rt::test]
async fn get_unknown_by_authentication() {
    let service = service::TestUsersService::new().await;

    let provider = "google".into();
    let provider_id = "123456".into();

    let user = service
        .users_service
        .get_by_authentication(&provider, &provider_id)
        .await;
    check!(user.is_none());
}

#[actix_rt::test]
async fn get_known_by_authentication() {
    let seeded_user = mire_testdata::SeedUser::default().with_authentication(
        "google",
        "123456",
        "test@example.com",
    );

    let service = service::TestUsersService::new().await;
    service.test_database.seed(&seeded_user).await;

    let provider = "google".into();
    let provider_id = "123456".into();

    let user = service
        .users_service
        .get_by_authentication(&provider, &provider_id)
        .await;

    assert!(user.is_some());

    let user = user.unwrap();
    check!(user.identity.id == seeded_user.user_id.into());
    check!(user.identity.version == seeded_user.version);
    check!(user.identity.created == seeded_user.created);
    check!(user.identity.updated == seeded_user.updated);
    check!(user.data.email == seeded_user.email.into());
    check!(user.data.display_name == seeded_user.display_name);
    check!(user.data.authentications.len() == 1);
    check!(user.data.authentications[0].authentication_provider == "google".into());
    check!(user.data.authentications[0].authentication_id == "123456".into());
    check!(user.data.authentications[0].display_name == "test@example.com");
}

#[actix_rt::test]
async fn get_unknown_by_authentication_wrong_provider() {
    let seeded_user = mire_testdata::SeedUser::default().with_authentication(
        "google",
        "123456",
        "test@example.com",
    );

    let service = service::TestUsersService::new().await;
    service.test_database.seed(&seeded_user).await;

    let provider = "other".into();
    let provider_id = "123456".into();

    let user = service
        .users_service
        .get_by_authentication(&provider, &provider_id)
        .await;
    check!(user.is_none());
}

#[actix_rt::test]
async fn get_unknown_by_authentication_wrong_provider_id() {
    let seeded_user = mire_testdata::SeedUser::default().with_authentication(
        "google",
        "123456",
        "test@example.com",
    );

    let service = service::TestUsersService::new().await;
    service.test_database.seed(&seeded_user).await;

    let provider = "google".into();
    let provider_id = "other".into();

    let user = service
        .users_service
        .get_by_authentication(&provider, &provider_id)
        .await;
    check!(user.is_none());
}

#[actix_rt::test]
async fn get_unknown_by_authentication_overlapping() {
    let seeded_user1 = mire_testdata::SeedUser::default().with_authentication(
        "google",
        "123456",
        "test@example.com",
    );

    let seeded_user2 = mire_testdata::SeedUser::default().with_authentication(
        "twitter",
        "098765",
        "test@example.com",
    );

    let service = service::TestUsersService::new().await;
    service.test_database.seed(&seeded_user1).await;
    service.test_database.seed(&seeded_user2).await;

    let provider = "google".into(); // From seeded_user1
    let provider_id = "098765".into(); // From seeded_user2

    let user = service
        .users_service
        .get_by_authentication(&provider, &provider_id)
        .await;
    check!(user.is_none());
}
