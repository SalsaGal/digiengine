mod system;

use crate::system::System;

use clap::Parser;
use current::*;

#[derive(Parser)]
struct Args {
    config: String,
}

fn main() {
    Digiengine::run();
}

struct Digiengine {
    system: System,
}

impl Game for Digiengine {
    fn init(data: &mut GameData) -> Self {
        data.graphics.frame_size = Some((2.0, 2.0).into());

        let args = Args::parse();
        let system_config = std::fs::read_to_string(&args.config).unwrap();
        let system = serde_json::from_str(&system_config).unwrap();

        dbg!(&system);

        Self {
            system,
        }
    }

    fn render<'a>(&'a mut self, _: graphics::Frame<'a>) {
    }
}
