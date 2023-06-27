use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use bevy::prelude::*;

pub const GRID_SIZE: usize = 25;

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
pub struct GridLocation(pub IVec2);

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
        GridLocation(IVec2::new(x as i32, y as i32))
    }
    pub fn from_world(position: Vec2) -> Option<Self> {
        let position = position + Vec2::splat(0.5);
        let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));
        if Grid::<()>::valid_index(&location) {
            Some(location)
        } else {
            None
        }
    }
}

impl From<IVec2> for GridLocation {
    fn from(value: IVec2) -> Self {
        GridLocation(value)
    }
}

impl<T> Grid<T> {
    pub fn occupied(&self, location: &GridLocation) -> bool {
        Grid::<T>::valid_index(location) && self[location].is_some()
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.x >= 0
            && location.y >= 0
            && location.x < GRID_SIZE as i32
            && location.y < GRID_SIZE as i32
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

fn remove_from_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    mut query: RemovedComponents<T>,
    mut dirty: EventWriter<DirtyGridEvent<T>>,
) {
    for removed_entity in query.iter() {
        // Search for entity
        let removed = grid.iter().find(|(entity, _)| *entity == removed_entity);
        if let Some((_, location)) = removed {
            dirty.send(DirtyGridEvent::<T>(
                location.clone(),
                PhantomData::default(),
            ));
            grid[&location] = None;
        }
    }
}

fn add_to_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    query: Query<(Entity, &GridLocation), Added<T>>,
    mut dirty: EventWriter<DirtyGridEvent<T>>,
) {
    for (entity, location) in &query {
        if let Some(existing) = grid[location] {
            if existing != entity {
                warn!("Over-writing entity in grid");
                dirty.send(DirtyGridEvent::<T>(
                    location.clone(),
                    PhantomData::default(),
                ));
                grid[location] = Some(entity);
            }
        } else {
            dirty.send(DirtyGridEvent::<T>(
                location.clone(),
                PhantomData::default(),
            ));
            grid[location] = Some(entity);
        }
    }
}

#[derive(Event)]
pub struct DirtyGridEvent<T>(pub GridLocation, PhantomData<T>);

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T: Component> Plugin for GridPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<T>>()
            .add_systems(Update, lock_to_grid::<T>)
            .add_event::<DirtyGridEvent<T>>()
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
