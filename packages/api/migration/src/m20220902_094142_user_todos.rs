use entity::todo::{Column, Entity as Todo};
use entity::user;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220902_094142_user_todos"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let todo_table = Table::alter()
            .table(Todo)
            .add_column(ColumnDef::new(Column::UserId).uuid().not_null())
            .to_owned();

        let user_id_fk = ForeignKey::create()
            .name("FK_0429e73d-e88e-4574-8db4-d0d45015eee0")
            .from(Todo, Column::UserId)
            .to(user::Entity, user::Column::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager.alter_table(todo_table).await?;
        manager.create_foreign_key(user_id_fk).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let todo_table = Table::alter()
            .table(Todo)
            .drop_column(Column::UserId)
            .to_owned();

        let drop_user_id_fk = ForeignKey::drop()
            .name("FK_0429e73d-e88e-4574-8db4-d0d45015eee0")
            .table(Todo)
            .to_owned();

        manager.alter_table(todo_table).await?;
        manager.drop_foreign_key(drop_user_id_fk).await
    }
}
