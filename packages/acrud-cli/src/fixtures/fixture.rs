use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};

#[async_trait(?Send)]
pub trait FixtureTrait {
    async fn exec(&self, conn: &DatabaseConnection) -> Result<(), DbErr>;
}

pub struct Fixture<Model: ActiveModelTrait> {
    pub name: String,
    dependencies: Vec<String>,
    items: Vec<Model>,
}

impl<Model: ActiveModelTrait> Fixture<Model> {
    pub fn new(name: String, dependencies: Vec<String>, items: Vec<Model>) -> Self {
        Self {
            name,
            dependencies,
            items,
        }
    }
}

#[async_trait(?Send)]
impl<Model: ActiveModelTrait> FixtureTrait for Fixture<Model> {
    async fn exec(&self, conn: &DatabaseConnection) -> Result<(), DbErr> {
        Model::Entity::insert_many(self.items.clone())
            .exec(conn)
            .await?;
        println!(
            "[{}] fixture loaded with {} items",
            self.name,
            self.items.len()
        );
        Ok(())
    }
}
