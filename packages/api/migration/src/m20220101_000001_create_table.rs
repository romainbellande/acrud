use entity::todo::Entity as Todo;
use sea_orm::{DbBackend, Schema};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_postgres = DbBackend::Postgres;
        let schema = Schema::new(db_postgres);
        let todo_table = schema.create_table_from_entity(Todo);
        // let sql = todo_table.build(PostgresQueryBuilder);
        // println!("todo sql: {:#?}", sql);
        manager.create_table(todo_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo).to_owned())
            .await
    }
}
