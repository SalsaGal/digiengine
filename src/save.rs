use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Save {
    creature: usize,
}
