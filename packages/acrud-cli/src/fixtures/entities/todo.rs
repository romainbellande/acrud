use entity::todo::{ActiveModel as TodoModel};
use sea_orm::{ActiveValue};
use crate::fixtures::Fixture;

pub fn todo_fixture() -> Box<Fixture<TodoModel>> {
    Box::new(
        Fixture::new(
        "todo".to_string(),
        vec!["foo".to_string()],
        vec![TodoModel{
            id: ActiveValue::Set(1),
            title: ActiveValue::Set("".to_string()),
            text: ActiveValue::Set("".to_string()),
        }],
    ))
}
