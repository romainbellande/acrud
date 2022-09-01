use entity::user::Entity as User;
use sea_orm::{DbBackend, Schema};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220827_111433_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_postgres = DbBackend::Postgres;
        let schema = Schema::new(db_postgres);
        let user_table = schema.create_table_from_entity(User);
        manager.create_table(user_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User).to_owned())
            .await
    }
}
