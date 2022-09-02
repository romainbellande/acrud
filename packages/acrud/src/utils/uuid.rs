use std::str::FromStr;

use hyper::StatusCode;
use uuid::Uuid;

use crate::errors::WebError;

pub fn str_to_uuid(value: String) -> Result<Uuid, WebError> {
    match Uuid::from_str(&value) {
        Ok(res) => Ok(res),
        Err(err) => Err(WebError {
            code: 12,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        }),
    }
}
