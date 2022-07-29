use acrud::errors::WebError;
use hyper::StatusCode;

pub enum TodoError {
    CouldNotSaveTodo,
}

// impl TodoError {
//     pub fn get(&self) -> WebError {
//         match self {
//             Self::CouldNotSaveTodo => WebError {
//                 code: 500,
//                 status: StatusCode::INTERNAL_SERVER_ERROR,
//                 message: "Could not save todo".to_string()
//             },
//         }
//     }
// }

impl From<TodoError> for WebError {
    fn from(error: TodoError) -> Self {
        match error {
            TodoError::CouldNotSaveTodo => WebError {
                code: 500,
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Could not save todo".to_string()
            },
        }
    }
}

