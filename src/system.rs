use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct System {
    pub name: String,
    pub stages: Vec<String>,
    pub creatures: Vec<Creature>,
    pub background: String,
}

#[derive(Debug, Deserialize)]
pub struct Creature {
    pub name: String,
    pub stage: usize,
}
