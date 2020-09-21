mod service;

use assert2::{check, let_assert};
use mire_world::StatId;

#[actix_rt::test]
async fn get_unknown_by_id() {
    let service = service::TestWorldService::new().await;

    let stat_id: StatId = "2ac84068-1664-4f41-8e98-a464c78c5e46".parse().unwrap();
    let stat = service.stats_service.get(&stat_id).await;
    check!(stat.is_none());
}

#[actix_rt::test]
#[allow(clippy::cast_sign_loss)]
async fn get_known_by_id() {
    let seeded_stat = mire_testdata::SeedStat::default();

    let service = service::TestWorldService::new().await;
    service.test_database.seed(&seeded_stat).await;

    let stat_id: StatId = seeded_stat.stat_id.into();
    let stat = service.stats_service.get(&stat_id).await;

    let_assert!(Some(stat) = stat);
    check!(stat.id == stat_id);
    check!(stat.name == seeded_stat.name);
    check!(stat.description == seeded_stat.description);
    check!(stat.default_value == seeded_stat.default_value as u32);
}
