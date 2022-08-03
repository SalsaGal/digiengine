use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct System {
    pub stages: Vec<String>,
}
