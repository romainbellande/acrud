use sea_orm::{entity::prelude::*, DeleteMany, Set};
use serde::{Deserialize, Serialize};
use utoipa::Component;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Component)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    #[component(example = "Buy fruits")]
    pub title: String,

    #[component(example = "3 apples, 2 bananas")]
    pub text: String,
}

impl Model {
    pub fn new(title: String, text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            text,
        }
    }

    pub fn into_active_model(&self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id.to_owned()),
            title: Set(self.title.to_owned()),
            text: Set(self.text.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Validate, Component)]
pub struct CreateTodo {
    #[component(example = "Buy fruits")]
    #[validate(length(min = 1))]
    pub title: String,

    #[component(example = "3 apples, 2 bananas")]
    #[validate(length(min = 1))]
    pub text: String,
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

    pub fn find_by_title(title: &str) -> Select<Entity> {
        Self::find().filter(Column::Title.eq(title))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
