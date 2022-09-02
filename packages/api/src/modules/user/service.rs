use super::errors::UserError;
use crate::config::CONFIG;
use acrud::errors::WebError;
use entity::user::{self, CreateUser, UserResponse};
use hyper::StatusCode;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use validator::Validate;

pub async fn create(
    create_user: CreateUser,
    conn: &DatabaseConnection,
) -> Result<UserResponse, WebError> {
    if let Err(err) = create_user.validate() {
        return Err(WebError {
            code: 400,
            message: err.to_string(),
            status: StatusCode::BAD_REQUEST,
        });
    }

    let user = user::Model::new(create_user.email, create_user.password, CONFIG.salt.clone());

    if user.into_active_model().insert(conn).await.is_err() {
        return Err(UserError::CouldNotSaveUser.into());
    }

    Ok(UserResponse::from(user))
}
