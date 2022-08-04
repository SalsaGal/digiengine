mod save;
mod system;

use std::fs::read_to_string;

use crate::system::System;

use clap::Parser;
use current::*;
use save::Save;

#[derive(Parser)]
struct Args {
    /// Location of system folder
    system_path: String,
    /// Alternative save file location (save.json in the system folder by default)
    save_path: Option<String>,
}

fn main() {
    Digiengine::run();
}

struct Digiengine {
    save: Option<Save>,
    system: System,
}

impl Game for Digiengine {
    fn init(data: &mut GameData) -> Self {
        data.graphics.frame_size = Some((2.0, 2.0).into());
        data.graphics.background_color = wgpu::Color::WHITE;

        let args = Args::parse();

        std::env::set_current_dir(args.system_path).unwrap();

        let system = serde_json::from_str(&read_to_string("system.json").unwrap()).unwrap();
        let save_path = args.save_path.unwrap_or_else(|| "save.json".to_owned());
        let save = if let Ok(contents) = read_to_string(save_path) {
            Some(serde_json::from_str(&contents).unwrap())
        } else {
            None
        };

        dbg!(&system);
        dbg!(&save);

        Self { save, system }
    }
}
