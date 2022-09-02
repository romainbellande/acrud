use serde::Deserialize;

#[derive(Deserialize)]
pub struct FindParams {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
