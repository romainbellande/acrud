use acrud::errors::WebError;
use hyper::StatusCode;

pub enum TodoError {
    CouldNotSaveTodo,
}

// impl From<TodoError> for WebError {
//     fn from(error: TodoError) -> Self {
//         match error {
//             TodoError::CouldNotSaveTodo => WebError {
//                 code: 500,
//                 status: StatusCode::INTERNAL_SERVER_ERROR,
//                 message: "Could not save todo".to_string()
//             },
//         }
//     }
// }

impl Into<WebError> for TodoError {
    fn into(self) -> WebError {
        match self {
            TodoError::CouldNotSaveTodo => WebError {
                code: 500,
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Could not save todo".to_string()
            }
        }
    }
}
