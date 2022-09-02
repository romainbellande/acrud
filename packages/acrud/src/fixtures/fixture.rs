use crate::utils::times;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait};

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
    pub fn new(
        name: String,
        dependencies: Vec<String>,
        nb: u32,
        items: fn(Vec<String>) -> Model,
    ) -> Self {
        Self {
            name,
            items: times(nb, |_| items(dependencies.clone())),
            dependencies,
        }
    }

    pub async fn clean(&self, conn: &DatabaseConnection) -> Result<DeleteResult, sea_orm::DbErr> {
        let result = Model::Entity::delete_many().exec(conn).await;

        match result.clone() {
            Ok(delete_resut) => {
                println!(
                    "[{}] table cleaned, {} rows affected",
                    self.name, delete_resut.rows_affected
                );
            }
            Err(err) => {
                println!("[{}] {}", self.name, err.to_string());
            }
        };

        return result;
    }

    pub fn to_box(self) -> Box<Fixture<Model>> {
        Box::new(self)
    }
}

#[async_trait(?Send)]
impl<Model: ActiveModelTrait> FixtureTrait for Fixture<Model> {
    async fn exec(&self, conn: &DatabaseConnection) -> Result<(), DbErr> {
        self.clean(conn).await?;

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
