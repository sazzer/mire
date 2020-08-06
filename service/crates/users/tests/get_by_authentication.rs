mod service;

use assert2::check;

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
