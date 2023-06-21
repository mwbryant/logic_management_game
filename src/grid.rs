use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use bevy::prelude::*;

pub const GRID_SIZE: usize = 12;

// TODO impl iterator
#[derive(Resource)]
pub struct Grid<T> {
    pub entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
    _marker: PhantomData<T>,
}

impl<T> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            entities: self.entities,
            _marker: self._marker,
        }
    }
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug, Deref, DerefMut)]
pub struct GridLocation(UVec2);

impl<T> Index<&GridLocation> for Grid<T> {
    type Output = Option<Entity>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.entities[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<&GridLocation> for Grid<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.entities[index.x as usize][index.y as usize]
    }
}

#[derive(Component)]
pub struct LockToGrid;

impl GridLocation {
    pub fn new(x: u32, y: u32) -> Self {
        GridLocation(UVec2::new(x, y))
    }
}

impl<T> Grid<T> {
    pub fn occupied(&self, location: &GridLocation) -> bool {
        self[location].is_some()
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Entity, GridLocation)> + '_ {
        self.entities
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_i, entity)| entity.is_some())
            .map(|(i, entity)| {
                (
                    entity.unwrap(),
                    GridLocation::new(i as u32 / GRID_SIZE as u32, i as u32 % GRID_SIZE as u32),
                )
            })
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
        if let Some(existing) = grid[location] {
            if existing != entity {
                grid[location] = Some(entity);
            }
        } else {
            grid[location] = Some(entity);
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
            .add_systems(Update, lock_to_grid::<T>)
            // TODO move_on_grid
            .add_systems(PreUpdate, (add_to_grid::<T>, remove_from_grid::<T>));
    }
}

// Could change detect
fn lock_to_grid<T: Component>(
    grid: Res<Grid<T>>,
    mut positions: Query<&mut Transform, (With<LockToGrid>, With<T>)>,
) {
    for (entity, location) in grid.iter() {
        if let Ok(mut position) = positions.get_mut(entity) {
            position.translation.x = location.x as f32;
            position.translation.y = location.y as f32;
        }
    }
}
