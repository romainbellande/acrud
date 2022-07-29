use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct WebError {
    pub code: u16,
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        (self.status, Json(json!({
            "code": self.code,
            "message": self.message,
            "status": self.status.as_u16(),
        }))).into_response()
    }
}
