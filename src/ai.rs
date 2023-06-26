use crate::prelude::*;
use rand::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (wander, update_brains, follow_path, get_food));
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

fn update_brains(mut brains: Query<(&mut Brain, &mut TextureAtlasSprite, &Hunger, &Recreation)>) {
    for (mut brain, mut sprite, hunger, recreation) in &mut brains {
        if hunger.value < 40.0 {
            brain.state = BrainState::GetFood;
            sprite.color = Color::ORANGE;
            continue;
        }
        /*
        if recreation.value < 0.4 {
            brain.state = BrainState::Relax;
            sprite.color = Color::BLUE;
            continue;
        }
        */

        if !matches!(brain.state, BrainState::Wander(_)) {
            sprite.color = Color::WHITE;
            brain.state = BrainState::Wander(0.0);
        }
    }
}

fn get_food(
    mut commands: Commands,
    mut brains: Query<(Entity, &mut Hunger, &AiPath, &Brain, &Transform), Without<PathfindingTask>>,
    walls: Res<Grid<Wall>>,
    food_grid: Res<Grid<FoodMachine>>,
    food: Query<&FoodMachine>,
) {
    for (target, mut hunger, path, brain, transform) in &mut brains {
        if matches!(brain.state, BrainState::GetFood) {
            if let Some((food_ent, location)) = food_grid.iter().next() {
                //TODO is there a way to not need these expects
                let food = food.get(food_ent).expect("Bad food in grid");

                let start = GridLocation::new(
                    transform.translation.x as u32,
                    transform.translation.y as u32,
                );

                let food_point = location.0 + food.use_offset;
                let food_transform = Vec2::new(food_point.x as f32, food_point.y as f32);

                if transform.translation.truncate().distance(food_transform) < 0.5 {
                    info!("Eating!");
                    hunger.value = 100.0;
                } else if path.locations.is_empty() {
                    info!("Getting food!");
                    spawn_optimized_pathfinding_task(
                        &mut commands,
                        target,
                        &walls,
                        &start,
                        &food_point.into(),
                    );
                }
            }
        }
    }
}

fn wander(
    mut commands: Commands,
    mut brains: Query<(Entity, &AiPath, &mut Brain, &Transform), Without<PathfindingTask>>,
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
fn follow_path(
    mut paths: Query<(&mut Transform, &mut AiPath, &mut LastDirection)>,
    time: Res<Time>,
) {
    for (mut transform, mut path, mut last_direction) in &mut paths {
        if let Some(next_target) = path.locations.front() {
            let delta = *next_target - transform.translation.truncate();
            let travel_amount = time.delta_seconds();

            if delta.length() > travel_amount * 1.1 {
                let direction = delta.normalize().extend(0.0) * travel_amount;
                last_direction.0 = direction.truncate();
                transform.translation += direction;
            } else {
                path.locations.pop_front();
            }
        } else {
            last_direction.0 = Vec2::ZERO;
        }
    }
}
