use super::errors::TodoError;
use acrud::errors::WebError;
use entity::todo::{self, CreateTodo};
use hyper::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use validator::Validate;

pub async fn create(
    create_todo: CreateTodo,
    conn: &DatabaseConnection,
) -> Result<todo::Model, WebError> {
    println!("create todo");
    if let Err(err) = create_todo.validate() {
        return Err(WebError {
            code: 400,
            message: err.to_string(),
            status: StatusCode::BAD_REQUEST,
        });
    }

    let todo = todo::Model::new(create_todo.title, create_todo.text);

    if todo.into_active_model().insert(conn).await.is_err() {
        return Err(TodoError::CouldNotSaveTodo.into());
    }

    Ok(todo)
}
