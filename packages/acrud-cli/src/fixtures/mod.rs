pub mod fixture;
pub mod service;
mod entities;

use fixture::{Fixture};
use service::FixtureService;
use entities::{todo::todo_fixture};
use acrud::db::Database;

pub async fn load() {
    let fixture_service = FixtureService {
        fixtures: vec![
            todo_fixture()
        ],
    };

    let database = Database::new().await;

    let conn = database.connection;

    fixture_service.exec(&conn).await;
}

