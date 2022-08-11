mod errors;
mod service;

use acrud::{extractors::json::Json, map_response::map_response, pagination::get_paginated_result};
use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use entity::todo::{self, CreateTodo, Entity as Todo};
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};
use serde::Deserialize;

const DEFAULT_LIMIT: usize = 20;
const DEFAULT_PAGE: usize = 1;

#[derive(Deserialize)]
pub struct Params {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[utoipa::path(
    get,
    path = "/api/todos",
    params(
        ("limit" = usize, path),
        ("page" = usize, path)
    ),
    responses(
        (status = 200, description = "List all todos successfully", body = [TodoModel])
    )
)]
pub async fn find(
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

#[utoipa::path(
    post,
    path = "/api/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "todo item created successfully", body = TodoModel),
        (status = 400, description = "bad request", body = WebError),
        (status = 500, description = "internal server error", body = WebError)
    )
)]
async fn create(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let result = service::create(payload, conn).await;
    map_response(result, Some(StatusCode::CREATED))
}

pub fn router() -> Router {
    Router::new().route("/", get(find).post(create))
}
