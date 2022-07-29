mod errors;
mod service;

use acrud::pagination::get_paginated_result;
use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
    routing::{get},
    Json, Router,
};
use entity::todo::{self, Entity as Todo, CreateTodo};
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, ActiveModelTrait};
use serde::Deserialize;
use validator::Validate;
use errors::TodoError;

const DEFAULT_LIMIT: usize = 20;
const DEFAULT_PAGE: usize = 1;

#[derive(Deserialize)]
struct Params {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

async fn find(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(DEFAULT_PAGE);
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT);

    let finder = Todo::find();
    let paginator = finder.paginate(conn, limit);

    let todos: Vec<todo::Model> = paginator
        .fetch_page(page - 1)
        .await
        .expect("could not retreive data");

    let paginated_result = get_paginated_result(paginator, todos).await;

    Json(paginated_result)
}

async fn create(Extension(ref conn): Extension<DatabaseConnection>, Json(payload): Json<CreateTodo>) -> impl IntoResponse {
    if let Err(err) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(err)).into_response();
    }

    let todo = todo::Model::new(payload.title, payload.text);

    if todo.into_active_model().insert(conn).await.is_err() {
        return TodoError::CouldNotSaveTodo.get().into_response();
    }

    (StatusCode::CREATED, Json(todo)).into_response()
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(find).post(create))
}
