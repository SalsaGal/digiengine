mod system;

use crate::system::System;

use clap::Parser;
use current::*;

#[derive(Parser)]
struct Args {
    /// The configuration file to read system information from
    config: String,
    /// The working directory to find system folders
    #[clap(long = "pwd")]
    working_dir: Option<String>,
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
        data.graphics.background_color = wgpu::Color::WHITE;

        let args = Args::parse();

        if let Some(dir) = args.working_dir {
            std::env::set_current_dir(dir).expect("Invalid working directory: {dir}");
        } else if !cfg!(debug_assertions) {
            let dir = std::env::current_exe().unwrap();
            std::env::set_current_dir(dir).unwrap();
        }

        let system_config = std::fs::read_to_string(&args.config).unwrap();
        let system = serde_json::from_str(&system_config).unwrap();

        dbg!(&system);

        Self { system }
    }

    fn render<'a>(&'a mut self, _: graphics::Frame<'a>) {}
}
