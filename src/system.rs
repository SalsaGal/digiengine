use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct System {
    pub name: String,
    pub stages: Vec<String>,
    pub creatures: Vec<Creature>,
}

#[derive(Debug, Deserialize)]
pub struct Creature {
    name: String,
    stage: usize,
}
