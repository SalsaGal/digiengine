use current::*;

fn main() {
    Digiengine::run();
}

struct Digiengine {}

impl Game for Digiengine {
    fn init(_: &mut GameData) -> Self {
        Self {}
    }
}
