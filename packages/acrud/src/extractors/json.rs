use std::error::Error;

use axum::{extract::{FromRequest, rejection::{JsonRejection, JsonDataError}, RequestParts}, BoxError, response::{IntoResponse, Response}, headers::HeaderValue};
use hyper::{StatusCode, header};
use serde::{de::DeserializeOwned, Serialize};
use crate::errors::WebError;
use serde_json::Value;
use axum::{async_trait};
use mime;

pub struct Json<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Json<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::Json`
    T: DeserializeOwned,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                // convert the error from `axum::Json` into whatever we want
                let web_error: WebError = match rejection {
                    JsonRejection::JsonDataError(err) => {
                        WebError {
                            code: 1,
                            message: json_data_error_to_string(err),
                            status: StatusCode::BAD_REQUEST,
                        }
                    },
                    JsonRejection::MissingJsonContentType(err) => WebError {
                        code: 2,
                        message: err.to_string(),
                        status: StatusCode::BAD_REQUEST,
                    },
                    err => WebError {
                        code: 3,
                        message: "internal server error".to_string(),
                        status: StatusCode::INTERNAL_SERVER_ERROR,
                    },
                };

                Err((web_error.status, web_error.into_json()))
            }
        }
    }
}

fn json_data_error_to_string(error: JsonDataError) -> String {
    match error.source() {
        Some(err) => {
            err.to_string()
        },
        None => {
            error.to_string()
        }
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                bytes,
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                WebError {
                    message: err.to_string(),
                    code: 1,
                    status: StatusCode::INTERNAL_SERVER_ERROR
                },
            )
                .into_response(),
        }
    }
}