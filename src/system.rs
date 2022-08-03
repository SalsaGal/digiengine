use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct System {
    pub name: String,
    pub stages: Vec<String>,
}
