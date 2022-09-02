use crate::auth::Claims;
use crate::modules::user::errors::UserError;

use super::errors::TodoError;
use super::FindParams;
use acrud::pagination::get_paginated_result;
use acrud::{errors::WebError, pagination::PaginatedResult, utils::uuid::str_to_uuid};
use entity::todo::{self, CreateTodo, Entity as Todo};
use entity::user::Entity as User;
use hyper::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, ModelTrait};
use sea_orm::{EntityTrait, PaginatorTrait};
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

const DEFAULT_LIMIT: usize = 20;
const DEFAULT_PAGE: usize = 1;

pub async fn find(
    conn: &DatabaseConnection,
    claims: Claims,
    params: FindParams,
) -> Result<PaginatedResult<todo::Model>, WebError> {
    let user_id = str_to_uuid(claims.sub)?;

    let user = User::find_by_id(user_id)
        .one(conn)
        .await
        .map_err(|err| WebError {
            code: 32,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?
        .ok_or_else(|| WebError {
            code: 33,
            status: StatusCode::NOT_FOUND,
            message: "user not found".to_string(),
        })?;

    let page = params.page.unwrap_or(DEFAULT_PAGE);
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT);
    let finder = user.find_related(Todo);
    let paginator = finder.paginate(conn, limit);

    let todos: Vec<todo::Model> = paginator
        .fetch_page(page - 1)
        .await
        .map_err(|err| WebError {
            code: 10,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?;

    Ok(get_paginated_result(paginator, todos).await)
}

pub async fn create(
    create_todo: CreateTodo,
    conn: &DatabaseConnection,
    user_id: String,
) -> Result<todo::Model, WebError> {
    if let Err(err) = create_todo.validate() {
        return Err(WebError {
            code: 400,
            message: err.to_string(),
            status: StatusCode::BAD_REQUEST,
        });
    }

    let user_id = Uuid::from_str(&user_id).map_err(|err| WebError {
        code: 4,
        status: StatusCode::BAD_REQUEST,
        message: format!("user id stored in jwt is not an uuid: {}", user_id),
    })?;

    let todo = todo::Model::new(create_todo.title, create_todo.text, user_id);

    todo.into_active_model()
        .insert(conn)
        .await
        .map_err(|err| WebError {
            code: 1,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        })?;

    Ok(todo)
}
