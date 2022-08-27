use sea_orm::{entity::prelude::*, DeleteMany, Set};
use serde::{Deserialize, Serialize};
use utoipa::Component;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Component)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    #[component(example = "john@example.com")]
    pub email: String,

    #[component(example = "hashpassword")]
    pub password_hash: String,
}

impl Model {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
        }
    }

    pub fn into_active_model(&self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id.to_owned()),
            email: Set(self.email.to_owned()),
            password_hash: Set(self.password_hash.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Validate, Component)]
pub struct CreateUser {
    #[component(example = "john@example.com")]
    #[validate(length(min = 1))]
    pub email: String,

    #[component(example = "myincrediblydifficultpassword")]
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_email(email: &str) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
