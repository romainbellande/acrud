pub mod user;

use acrud::fixtures::FixtureService;

pub fn service() -> FixtureService {
    FixtureService {
        fixtures: vec![user::fixture()],
    }
}
