use crate::config::CONFIG;
use acrud::fixtures::Fixture;
use entity::user::{self, ActiveModel as UserModel};
use fake::faker::lorem::en::{Sentence, Word};
use fake::Fake;
use sea_orm::{ActiveModelBehavior, ActiveValue};

pub fn fixture() -> Box<Fixture<UserModel>> {
    Fixture::new("user".to_string(), vec![], 1, |_| {
        user::Model::new(
            "john@example.com".to_string(),
            "mystrongpassword".to_string(),
            CONFIG.salt.clone(),
        )
        .into_active_model()
    })
    .to_box()
}
