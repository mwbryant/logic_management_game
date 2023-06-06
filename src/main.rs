use std::marker::PhantomData;

use bevy::{prelude::*, window::PresentMode};

pub const WIDTH: f32 = 640.0;
pub const HEIGHT: f32 = 480.0;
pub const GRID_SIZE: usize = 48;

// TODO impl iterator
#[derive(Resource)]
pub struct Grid<T> {
    // Problems:
    // Entites may be despawned and would leave bad refrence
    // Entities may have never been added to the right grid
    entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
    _marker: PhantomData<T>,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            entities: [[None; GRID_SIZE]; GRID_SIZE],
            _marker: Default::default(),
        }
    }
}

fn remove_from_grid<T: Component>(mut grid: ResMut<Grid<T>>, query: Query<&T>) {
    for entity in grid
        .entities
        .iter_mut()
        .flatten()
        .filter(|entity| entity.is_some())
    {
        if !query.contains(entity.unwrap()) {
            *entity = None;
        }
    }
}

fn react_on_removal(removed: RemovedComponents<MyComponent>, mut query: Query<&mut Sprite>) {
    // `RemovedComponents<T>::iter()` returns an interator with the `Entity`s that had their
    // `Component` `T` (in this case `MyComponent`) removed at some point earlier during the frame.
    for entity in removed.iter() {
        if let Ok(mut sprite) = query.get_mut(entity) {
            sprite.color.set_r(0.0);
        }
    }
}

#[derive(Component)]
pub struct GridLocation {
    position: (usize, usize),
}

fn add_to_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    query: Query<(Entity, &GridLocation), With<T>>,
) {
    for (entity, location) in &query {
        if let Some(existing) = grid.entities[location.position.0][location.position.1] {
            if existing != entity {
                grid.entities[location.position.0][location.position.1] = Some(entity);
            }
        } else {
            grid.entities[location.position.0][location.position.1] = Some(entity);
        }
    }
}

// Is default really required
#[derive(Component, Default, Debug)]
pub struct Wall {
    health: f32,
}

fn use_grid(grid: Res<Grid<Wall>>, walls: Query<&Wall>) {
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
    }
}

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T: Component> Plugin for GridPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<T>>()
            .add_systems(Update, (add_to_grid::<T>, remove_from_grid::<T>));
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
