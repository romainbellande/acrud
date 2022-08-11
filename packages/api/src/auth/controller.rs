use super::{
    body::AuthBody, claims::Claims, credentials::Credentials, errors::AuthError, keys::KEYS,
};
use axum::{Json, routing::post, Router};
use jsonwebtoken::{encode, Header};
use acrud::errors::WebError;

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = Credentials,
    responses(
        (status = 200, description = "conneciton successful", body = AuthBody),
        (status = 400, description = "missing credentials", body = WebError),
        (status = 401, description = "unauthorized", body = WebError),
        (status = 500, description = "internal server error", body = WebError)
    )
)]
pub async fn authorize(Json(credentials): Json<Credentials>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if credentials.email != "foo" || credentials.password != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

pub fn router() -> Router {
    Router::new().route("/", post(authorize))
}