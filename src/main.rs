mod save;
mod system;

use std::path::PathBuf;

use crate::system::System;

use clap::Parser;
use current::*;
use save::Save;

#[derive(Parser)]
struct Args {
    /// The configuration file to read system information from
    config: String,
    /// The working directory to find system folders
    #[clap(long = "pwd")]
    working_dir: Option<String>,
    /// The path of the save data
    #[clap(short = 's', long = "save")]
    save: Option<String>,
}

fn main() {
    Digiengine::run();
}

struct Digiengine {
    save: Save,
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

        let save_path = if let Some(path) = args.save {
            PathBuf::from(path)
        } else {
            let mut dir = std::env::current_exe().unwrap();
            dir.push("save.json");
            dir
        };
        let save_contents = std::fs::read_to_string(save_path).unwrap();
        let save = serde_json::from_str(&save_contents).unwrap();

        dbg!(&system);
        dbg!(&save);

        Self {
            save,
            system
        }
    }

    fn render<'a>(&'a mut self, _: graphics::Frame<'a>) {}
}
