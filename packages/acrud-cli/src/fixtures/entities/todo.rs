use crate::fixtures::Fixture;
use entity::todo::ActiveModel as TodoModel;
use sea_orm::ActiveValue;

pub fn todo_fixture() -> Box<Fixture<TodoModel>> {
    Box::new(Fixture::new(
        "todo".to_string(),
        vec!["foo".to_string()],
        vec![TodoModel {
            title: ActiveValue::Set("".to_string()),
            text: ActiveValue::Set("".to_string()),
            ..Default::default()
        }],
    ))
}
