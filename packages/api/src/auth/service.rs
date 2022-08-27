use super::{
    body::AuthBody, claims::Claims, credentials::Credentials, errors::AuthError, keys::KEYS,
};
use acrud::errors::WebError;
use jsonwebtoken::{encode, Header};

pub async fn authorize(credentials: Credentials) -> Result<AuthBody, WebError> {
    // Check if the user sent the credentials
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }
    // Here you can check the user credentials from a database
    if credentials.email != "foo@example.com" || credentials.password != "bar" {
        return Err(AuthError::WrongCredentials.into());
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation.into())?;

    Ok(AuthBody::new(token))
}
