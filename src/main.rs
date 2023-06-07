use rand::prelude::*;
use std::collections::VecDeque;

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

pub const WIDTH: f32 = 640.0;
pub const HEIGHT: f32 = 480.0;

mod grid;
use grid::{Grid, GridLocation, GridPlugin};

// Is default really required
#[derive(Component, Default, Debug)]
pub struct Wall {
    health: f32,
}

fn use_grid(
    grid: Res<Grid<Wall>>,
    walls: Query<&Wall>,
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for entity in grid
        .entities
        .iter()
        // This handles the double array
        .flatten()
        // This kills the option
        .flatten()
    {
        let wall = walls
            .get(*entity)
            .expect("entity in grid does not have wall component");
        info!("{:?}", wall);
        if keyboard.just_pressed(KeyCode::A) {
            commands.entity(*entity).despawn();
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
    last_wander_time: f32,
}

#[derive(Component, Default)]
pub struct Path {
    locations: VecDeque<Vec2>,
}

fn spawn_pawns(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 16.0,
        min_height: 9.0,
    };

    commands.spawn(camera);

    for _i in 0..10 {
        commands.spawn((
            SpriteBundle::default(),
            Pawn,
            Brain::default(),
            Path::default(),
            Hunger { value: 100.0 },
            Recreation { value: 100.0 },
        ));
    }
}

fn wander(mut brains: Query<(&mut Path, &mut Brain)>, time: Res<Time>) {
    for (mut path, mut brain) in &mut brains {
        brain.last_wander_time += time.delta_seconds();
        if brain.last_wander_time > 10.0 || path.locations.is_empty() {
            brain.last_wander_time = 0.0;

            let mut rng = rand::thread_rng();
            let x = rng.gen::<f32>() * 10.0 - 5.0;
            let y = rng.gen::<f32>() * 10.0 - 5.0;

            path.locations.push_back(Vec2::new(x, y));
        }
    }
}

// Does this need to read global transform
fn follow_path(mut paths: Query<(&mut Transform, &mut Path)>, time: Res<Time>) {
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
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(GridPlugin::<Wall>::default())
        .add_systems(Update, (use_grid, follow_path, wander))
        .add_systems(Startup, (spawn_walls, spawn_pawns))
        .run();
}

fn spawn_walls(mut commands: Commands) {
    commands.spawn((Wall { health: 10.0 }, GridLocation { position: (0, 0) }));
    commands.spawn((Wall { health: 25.0 }, GridLocation { position: (15, 15) }));
}
