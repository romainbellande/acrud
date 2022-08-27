use serde::Deserialize;
use utoipa::Component;
#[derive(Debug, Deserialize, Component)]
pub struct Credentials {
    #[component(example = "foo@example.com")]
    pub email: String,

    #[component(example = "bar")]
    pub password: String,
}
