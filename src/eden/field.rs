use crate::eden::grid::World;

pub struct FieldSystem {}

impl FieldSystem {
    pub fn new() -> Self {
        FieldSystem {}
    }

    pub fn update(&mut self, _world: &mut World) {
        // TODO: drift/decay memory later
    }
}
