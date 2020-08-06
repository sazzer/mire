mod service;

use assert2::check;
use mire_users::UserId;

#[actix_rt::test]
async fn get_unknown_by_id() {
    let service = service::TestUsersService::new().await;

    let user_id: UserId = "2ac84068-1664-4f41-8e98-a464c78c5e46".parse().unwrap();
    let user = service.users_service.get_by_id(&user_id).await;
    check!(user.is_none());
}

#[actix_rt::test]
async fn get_known_by_id() {
    let seeded_user = mire_testdata::SeedUser {
        ..mire_testdata::SeedUser::default()
    };

    let service = service::TestUsersService::new().await;
    service.test_database.seed(&seeded_user).await;

    let user_id: UserId = seeded_user.user_id.into();
    let user = service.users_service.get_by_id(&user_id).await;

    check!(user.is_some());

    let user = user.unwrap();
    check!(user.identity.id == user_id);
    check!(user.identity.version == seeded_user.version);
    check!(user.identity.created == seeded_user.created);
    check!(user.identity.updated == seeded_user.updated);
    check!(user.data.email == seeded_user.email.into());
    check!(user.data.display_name == seeded_user.display_name);
}
