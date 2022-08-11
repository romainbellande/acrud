use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::{json, Value};
use utoipa::Component;

#[derive(Debug, Clone, Component)]
pub struct WebError {
    #[component(example = 12)]
    pub code: u16,

    #[component(example = 400)]
    pub status: StatusCode,

    #[component(example = "bad request")]
    pub message: String,
}

impl WebError {
    pub fn into_json(&self) -> Json<Value> {
        Json(json!({
            "code": self.code,
            "message": self.message,
            "status": self.status.as_u16(),
        }))
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        (self.status, self.into_json()).into_response()
    }
}
