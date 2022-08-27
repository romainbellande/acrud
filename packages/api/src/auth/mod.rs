pub mod body;
mod claims;
pub mod credentials;
mod errors;
mod keys;
mod service;

use credentials::Credentials;
use acrud::{map_response::map_response};
use axum::{response::IntoResponse, routing::post, Json, Router};


#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = Credentials,
    responses(
        (status = 200, description = "connection successful", body = AuthBody),
        (status = 400, description = "missing credentials", body = WebError),
        (status = 401, description = "unauthorized", body = WebError),
        (status = 500, description = "internal server error", body = WebError)
    )
)]
pub async fn authorize(Json(credentials): Json<Credentials>) -> impl IntoResponse {
    let result = service::authorize(credentials).await;
    tracing::debug!("result: {:?}", result);
    map_response(result, None)
}

pub fn router() -> Router {
    Router::new().route("/login", post(authorize))
}
