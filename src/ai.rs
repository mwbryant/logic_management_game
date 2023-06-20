use crate::prelude::*;
use rand::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (wander, update_brains, follow_path));
    }
}

#[derive(Component)]
pub struct Pawn;

#[derive(Component, Default)]
pub struct Brain {
    state: BrainState,
}

pub enum BrainState {
    Wander(f32),
    GetFood,
    Relax,
}

impl Default for BrainState {
    fn default() -> Self {
        BrainState::Wander(0.0)
    }
}

fn update_brains(mut brains: Query<(&mut Brain, &mut Sprite, &Hunger, &Recreation)>) {
    for (mut brain, mut sprite, hunger, recreation) in &mut brains {
        if hunger.value < 0.4 {
            brain.state = BrainState::GetFood;
            sprite.color = Color::ORANGE;
            continue;
        }
        if recreation.value < 0.4 {
            brain.state = BrainState::Relax;
            sprite.color = Color::BLUE;
            continue;
        }

        if !matches!(brain.state, BrainState::Wander(_)) {
            sprite.color = Color::WHITE;
            brain.state = BrainState::Wander(0.0);
        }
    }
}

fn wander(
    mut commands: Commands,
    mut brains: Query<(Entity, &AiPath, &mut Brain, &Transform)>,
    time: Res<Time>,
    walls: Res<Grid<Wall>>,
) {
    for (target, path, mut brain, transform) in &mut brains {
        if let BrainState::Wander(last_wander_time) = &mut brain.state {
            *last_wander_time += time.delta_seconds();
            if *last_wander_time > 1.0 && path.locations.is_empty() {
                *last_wander_time = 0.0;

                let mut rng = rand::thread_rng();
                let x = rng.gen::<u32>() % GRID_SIZE as u32;
                let y = rng.gen::<u32>() % GRID_SIZE as u32;

                let start = GridLocation::new(
                    transform.translation.x as u32,
                    transform.translation.y as u32,
                );
                let end = GridLocation::new(x, y);
                spawn_optimized_pathfinding_task(&mut commands, target, &walls, &start, &end);
            }
        }
    }
}

// Does this need to read global transform
fn follow_path(mut paths: Query<(&mut Transform, &mut AiPath)>, time: Res<Time>) {
    for (mut transform, mut path) in &mut paths {
        if let Some(next_target) = path.locations.front() {
            let delta = *next_target - transform.translation.truncate();
            let travel_amount = time.delta_seconds();

            if delta.length() > travel_amount * 1.1 {
                transform.translation += delta.normalize().extend(0.0) * travel_amount;
            } else {
                path.locations.pop_front();
            }
        }
    }
}
