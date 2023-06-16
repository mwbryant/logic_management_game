use rand::prelude::*;

use bevy::{render::camera::ScalingMode, window::PresentMode};
use logic_management_tutorial::prelude::*;

pub const WIDTH: f32 = 1920.0;
pub const HEIGHT: f32 = 1080.0;

// Is default really required
#[derive(Component, Default, Debug)]
pub struct Wall {
    _health: f32,
}

fn use_grid(
    grid: Res<Grid<Wall>>,
    walls: Query<&Wall>,
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, _) in grid.iter() {
        let _wall = walls
            .get(entity)
            .expect("entity in grid does not have wall component");
        if keyboard.just_pressed(KeyCode::A) {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct Pawn;

// TODO abstract into a needs trait?
#[derive(Component)]
pub struct Hunger {
    value: f32,
}

#[derive(Component)]
pub struct Recreation {
    value: f32,
}

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

fn spawn_pawns(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 64.0,
        min_height: 36.0,
    };

    commands.spawn(camera);

    for _i in 0..1 {
        info!("Spawning");
        commands.spawn((
            SpatialBundle::default(),
            CharacterSprite::Stand(Facing::Down),
            Pawn,
            Brain::default(),
            AiPath::default(),
            Hunger { value: 100.0 },
            Recreation { value: 100.0 },
        ));
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

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Immediate,
                        title: "Logic Management Game".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(GridPlugin::<Wall>::default())
        .add_plugin(GraphicsPlugin)
        .add_systems(
            Update,
            (
                use_grid,
                follow_path,
                wander,
                update_brains,
                apply_pathfinding,
                apply_hunger,
                apply_recreation,
            ),
        )
        .add_systems(Startup, (spawn_walls, spawn_pawns))
        .run();
}

// Could be generic needs system
fn apply_hunger(mut hungers: Query<&mut Hunger>, time: Res<Time>) {
    for mut hunger in &mut hungers {
        hunger.value -= time.delta_seconds() * 10.0;
    }
}

fn apply_recreation(mut recreations: Query<&mut Recreation>, time: Res<Time>) {
    for mut recreations in &mut recreations {
        recreations.value -= time.delta_seconds() * 10.0;
    }
}

fn spawn_walls(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn((
            SpatialBundle::default(),
            Wall { _health: 10.0 },
            LockToGrid,
            WallSprite::Neutral,
            GridLocation::new(3, i),
        ));
    }
}
