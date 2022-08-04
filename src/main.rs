mod save;
mod system;

use std::fs::read_to_string;

use crate::system::System;

use clap::Parser;
use current::{*, sprite::{Sprite, Transform}};
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

    background: Sprite,
}

impl Game for Digiengine {
    fn init(data: &mut GameData) -> Self {
        data.graphics.frame_size = Some((2.0, 2.0).into());
        data.graphics.background_color = wgpu::Color::WHITE;

        let args = Args::parse();

        std::env::set_current_dir(args.system_path).unwrap();

        let system: System = serde_json::from_str(&read_to_string("system.json").unwrap()).unwrap();
        let save_path = args.save_path.unwrap_or_else(|| "save.json".to_owned());
        let save = if let Ok(contents) = read_to_string(save_path) {
            Some(serde_json::from_str(&contents).unwrap())
        } else {
            None
        };

        let background = Sprite::new_path_rect(data.graphics, &system.background, sprite::Filter::Nearest)
            .with_transform(Transform::scale((2.0, 2.0).into()).with_translation((0.0, 0.0, 0.0).into()));

        dbg!(&system);
        dbg!(&save);

        Self {
            save,
            system,

            background,
        }
    }

    fn render<'a>(&'a mut self, mut frame: graphics::Frame<'a>) {
        self.background.render_to(&mut frame);
    }
}
