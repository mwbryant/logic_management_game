use crate::prelude::*;

pub struct NeedsPlugin;

impl Plugin for NeedsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_hunger, apply_recreation));
    }
}

// TODO abstract into a needs trait?
#[derive(Component)]
pub struct Hunger {
    pub value: f32,
}

#[derive(Component)]
pub struct Recreation {
    pub value: f32,
}

// Could be generic needs system
fn apply_hunger(mut hungers: Query<&mut Hunger>, time: Res<Time>) {
    for mut hunger in &mut hungers {
        hunger.value -= time.delta_seconds() * 3.0;
    }
}

fn apply_recreation(mut recreations: Query<&mut Recreation>, time: Res<Time>) {
    for mut recreations in &mut recreations {
        recreations.value -= time.delta_seconds() * 10.0;
    }
}
