use entity::todo::{Entity as Todo, ActiveModel as TodoModel};
use sea_orm::{DatabaseConnection, EntityTrait, DbErr, InsertResult, ActiveModelTrait, ActiveValue};
use crate::db::Database;

pub struct Fixture<Model: ActiveModelTrait> {
    name: String,
    dependencies: Vec<String>,
    items: Vec<Model>,
}

impl <Model: ActiveModelTrait>Fixture<Model> {
    pub fn new(name: String, dependencies: Vec<String>, items: Vec<Model>) -> Self {
        Self {
            name,
            dependencies,
            items,
        }
    }

    pub async fn exec(&self, conn: &DatabaseConnection) -> Result<InsertResult<Model>, DbErr> {
        Ok(Model::Entity::insert_many(self.items.clone()).exec(conn).await?)
    }
}

pub async fn exec() {
    let db =  Database::new().await;
    let connection = db.connection;

    let todos: Vec<TodoModel> = vec![
        TodoModel {
            title: ActiveValue::Set("".to_string()),
            text: ActiveValue::Set("".to_string()),
            ..Default::default()
        }
    ];

    let fixture = Fixture::new("todo".to_owned(), vec![], todos);

    fixture.exec(&connection).await;
}
