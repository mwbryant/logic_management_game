use crate::prelude::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GridPlugin::<Wall>::default(),
            GridPlugin::<Machine>::default(),
        ));
    }
}

// Is default really required
#[derive(Component, Default, Debug)]
pub struct Wall {
    pub _health: f32,
}

#[derive(Component, Default, Debug)]
pub struct Machine {
    pub use_offset: IVec2,
}

#[derive(Component, Default, Debug)]
pub struct FoodMachine;
