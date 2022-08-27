use acrud::errors::WebError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl Into<WebError> for AuthError {
    fn into(self) -> WebError {
        match self {
            Self::WrongCredentials => WebError {
                code: 1,
                status: StatusCode::UNAUTHORIZED,
                message: "Wrong credentials".to_string(),
            },
            Self::MissingCredentials => WebError {
                code: 2,
                status: StatusCode::BAD_REQUEST,
                message: "Missing credentials".to_string(),
            },
            Self::TokenCreation => WebError {
                code: 3,
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Token creation error".to_string(),
            },
            Self::InvalidToken => WebError {
                code: 4,
                status: StatusCode::BAD_REQUEST,
                message: "Invalid token".to_string(),
            },
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
