use super::fixture::FixtureTrait;
use futures::future;
use sea_orm::DatabaseConnection;

pub struct FixtureService {
    pub fixtures: Vec<Box<dyn FixtureTrait>>,
}

impl FixtureService {
    pub async fn exec(&self, conn: &DatabaseConnection) {
        future::join_all(self.fixtures.iter().map(|f| f.exec(conn))).await;
    }
}
