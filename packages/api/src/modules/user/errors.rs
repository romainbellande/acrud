use acrud::errors::WebError;
use hyper::StatusCode;

pub enum UserError {
    CouldNotSaveUser,
    NotFound { email: String },
}

impl Into<WebError> for UserError {
    fn into(self) -> WebError {
        match self {
            Self::CouldNotSaveUser => WebError {
                code: 500,
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "could not save user".to_string(),
            },
            Self::NotFound { email } => WebError {
                code: 404,
                status: StatusCode::NOT_FOUND,
                message: format!("user with email {} not found", email),
            },
        }
    }
}
