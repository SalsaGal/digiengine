use current::*;

fn main() {
    Digiengine::run();
}

struct Digiengine {}

impl Game for Digiengine {
    fn init(data: &mut GameData) -> Self {
        data.graphics.frame_size = Some((2.0, 2.0).into());
        Self {}
    }
}
