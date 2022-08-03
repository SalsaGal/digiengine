mod system;

use crate::system::System;

use current::*;
use serde::Deserialize;

fn main() {
    Digiengine::run();
}

struct Digiengine {
    system: System,
}

impl Game for Digiengine {
    fn init(data: &mut GameData) -> Self {
        data.graphics.frame_size = Some((2.0, 2.0).into());
        let system_data = r#"{ "test": 5 }"#;
        Self {
            system: serde_json::from_str(system_data).unwrap(),
        }
    }
}
