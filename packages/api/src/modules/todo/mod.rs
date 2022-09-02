mod errors;
mod params;
mod service;
use crate::auth::{middleware::auth_bearer_middleware, Claims};
use acrud::{
    map_response::map_response, pagination::get_paginated_result, utils::uuid::str_to_uuid,
};
use axum::{
    extract::{Extension, Query},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use entity::todo::{self, CreateTodo, Entity as Todo};
use entity::user::Entity as User;
use hyper::StatusCode;
use params::FindParams;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};
use serde::Deserialize;

#[utoipa::path(
    get,
    path = "/api/todos",
    params(
        ("limit" = Option<usize>, query),
        ("page" = Option<usize>, query)
    ),
    responses(
        (status = 200, description = "List all todos successfully", body = [TodoModel])
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn find(
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<FindParams>,
) -> impl IntoResponse {
    let response = service::find(conn, claims, params).await;
    map_response(response, None)
}

#[utoipa::path(
    post,
    path = "/api/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "todo item created successfully", body = TodoModel),
        (status = 400, description = "bad request", body = WebError),
        (status = 500, description = "internal server error", body = WebError)
    ),
    security(
        ("authorization" = [])
    )
)]
pub async fn create(
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    tracing::debug!("claims: {:?}", claims);
    let result = service::create(payload, conn, claims.sub).await;
    map_response(result, Some(StatusCode::CREATED))
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(find).post(create))
        .route_layer(middleware::from_fn(auth_bearer_middleware))
}
