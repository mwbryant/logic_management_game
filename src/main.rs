use bevy::{prelude::*, window::PresentMode};

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
        .add_systems(Update, use_grid)
        .add_systems(Startup, spawn_walls)
        .run();
}

fn spawn_walls(mut commands: Commands) {
    commands.spawn((Wall { health: 10.0 }, GridLocation { position: (0, 0) }));
    commands.spawn((Wall { health: 25.0 }, GridLocation { position: (15, 15) }));
}
