use std::marker::PhantomData;

use bevy::prelude::*;

pub const GRID_SIZE: usize = 48;

// TODO impl iterator
#[derive(Resource)]
pub struct Grid<T> {
    // Problems:
    // Entites may be despawned and would leave bad refrence - handled by removed system
    // Entities may have never been added to the right grid - handled by add system
    pub entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
    _marker: PhantomData<T>,
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GridLocation {
    pub position: (usize, usize),
}

impl GridLocation {
    pub fn new(x: usize, y: usize) -> Self {
        GridLocation { position: (x, y) }
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            entities: [[None; GRID_SIZE]; GRID_SIZE],
            _marker: Default::default(),
        }
    }
}

fn remove_from_grid<T: Component>(mut grid: ResMut<Grid<T>>, mut query: RemovedComponents<T>) {
    for removed_entity in query.iter() {
        // Search for entity
        if let Some(entity) = grid
            .entities
            .iter_mut()
            .flatten()
            .filter(|entity| entity.is_some())
            .find(|entity| entity.unwrap() == removed_entity)
        {
            *entity = None;
        }
    }
}

fn add_to_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    query: Query<(Entity, &GridLocation), Added<T>>,
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

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T: Component> Plugin for GridPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<T>>()
            .add_systems(PreUpdate, (add_to_grid::<T>, remove_from_grid::<T>));
    }
}
