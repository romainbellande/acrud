use serde::Deserialize;
use utoipa::Component;

#[derive(Debug, Deserialize, Component)]
pub struct Credentials {
    #[component(example = "john@example.com")]
    pub email: String,

    #[component(example = "mystrongpassword")]
    pub password: String,
}
