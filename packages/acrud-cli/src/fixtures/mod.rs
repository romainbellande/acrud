mod entities;
pub mod fixture;
pub mod service;

use acrud::db::Database;
use entities::todo::todo_fixture;
use fixture::Fixture;
use service::FixtureService;

pub async fn load() {
    let fixture_service = FixtureService {
        fixtures: vec![todo_fixture()],
    };

    let database = Database::new().await;

    let conn = database.connection;

    fixture_service.exec(&conn).await;
}
