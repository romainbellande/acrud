use std::env;
use once_cell::sync::Lazy;
pub struct Config {
    pub jwt_secret: String
}

impl Config {
    pub fn new() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set")
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::new()
});

