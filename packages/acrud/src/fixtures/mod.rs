mod fixture;
mod service;

pub use fixture::{Fixture, FixtureTrait};
pub use service::FixtureService;

use crate::db::Database;

pub async fn load(fixture_service: FixtureService) {
    // let fixture_service = FixtureService {
    //     fixtures: vec![todo::fixture()],
    // };

    let database = Database::new().await;

    let conn = database.connection;

    fixture_service.exec(&conn).await;
}
