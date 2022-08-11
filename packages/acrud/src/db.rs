use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        let database_url = &std::env::var("DATABASE_URL").unwrap();

        let connection = sea_orm::Database::connect(database_url)
            .await
            .unwrap_or_else(|_| panic!("could not connect to database: {}", database_url));
        Database { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
