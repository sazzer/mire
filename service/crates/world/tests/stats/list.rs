use crate::service;
use assert2::check;

#[actix_rt::test]
async fn list_no_stats() {
    let service = service::TestWorldService::new().await;

    let stats = service.stats_service.list().await;

    check!(stats.total == 0);
    check!(stats.offset == 0);
    check!(stats.data.len() == 0);
}

#[actix_rt::test]
async fn list_one_stat() {
    let strength = mire_testdata::SeedStat {
        name: "Strength".to_owned(),
        ..mire_testdata::SeedStat::default()
    };

    let service = service::TestWorldService::new().await;
    service.test_database.seed(&strength).await;

    let stats = service.stats_service.list().await;

    check!(stats.total == 1);
    check!(stats.offset == 0);
    check!(stats.data.len() == 1);

    check!(stats[0].name == strength.name);
    check!(stats[0].description == strength.description);
}

#[actix_rt::test]
async fn list_several_stats() {
    let strength = mire_testdata::SeedStat {
        name: "Strength".to_owned(),
        ..mire_testdata::SeedStat::default()
    };
    let dexterity = mire_testdata::SeedStat {
        name: "Dexterity".to_owned(),
        ..mire_testdata::SeedStat::default()
    };
    let intelligence = mire_testdata::SeedStat {
        name: "Intelligence".to_owned(),
        ..mire_testdata::SeedStat::default()
    };

    let service = service::TestWorldService::new().await;
    service.test_database.seed(&strength).await;
    service.test_database.seed(&dexterity).await;
    service.test_database.seed(&intelligence).await;

    let stats = service.stats_service.list().await;

    check!(stats.total == 3);
    check!(stats.offset == 0);
    check!(stats.data.len() == 3);

    check!(stats[0].name == dexterity.name);
    check!(stats[1].name == intelligence.name);
    check!(stats[2].name == strength.name);
}
