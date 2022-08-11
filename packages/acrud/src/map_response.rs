use crate::errors::WebError;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

pub fn map_response<T: Serialize>(
    response: Result<T, WebError>,
    status: Option<StatusCode>,
) -> Response {
    match response {
        Ok(result) => {
            let final_status = match status {
                Some(status) => status,
                None => StatusCode::OK,
            };

            (final_status, Json(result)).into_response()
        }
        Err(web_error) => web_error.into_response(),
    }
}
