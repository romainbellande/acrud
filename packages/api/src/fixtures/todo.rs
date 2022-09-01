use acrud::fixtures::Fixture;
use entity::todo::ActiveModel as TodoModel;
use fake::faker::lorem::en::{Sentence, Word};
use fake::Fake;
use sea_orm::ActiveValue;

pub fn fixture() -> Box<Fixture<TodoModel>> {
    Fixture::new("todo".to_string(), vec!["foo".to_string()], 50, |_| {
        TodoModel {
            title: ActiveValue::Set(Word().fake::<String>()),
            text: ActiveValue::Set(Sentence(3..5).fake::<String>()),
            ..Default::default()
        }
    })
    .to_box()
}
