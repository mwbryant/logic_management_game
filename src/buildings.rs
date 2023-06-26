use crate::prelude::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GridPlugin::<Wall>::default());
    }
}

// Is default really required
#[derive(Component, Default, Debug)]
pub struct Wall {
    pub _health: f32,
}
