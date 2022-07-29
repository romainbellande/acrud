use acrud::errors::WebError;
use entity::todo::{self, Entity as Todo, CreateTodo};

pub fn create(create_todo: CreateTodo) -> Result<todo::Model, WebError> {
    if let Err(err) = create_todo.validate() {
        return (StatusCode::BAD_REQUEST, Json(err)).into_response();
    }
    let todo = todo::Model::new(create_todo.title, create_todo.text);
}
