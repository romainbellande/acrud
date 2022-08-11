use acrud::errors::WebError;
use hyper::StatusCode;

pub enum TodoError {
    CouldNotSaveTodo,
}

impl Into<WebError> for TodoError {
    fn into(self) -> WebError {
        match self {
            Self::CouldNotSaveTodo => WebError {
                code: 500,
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Could not save todo".to_string(),
            },
        }
    }
}
