use std::{
    collections::HashSet,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use pathfinding::undirected::connected_components;
use rand::{seq::IteratorRandom, Rng};

use crate::prelude::neumann_neighbors;

pub const GRID_SIZE: usize = 200;

#[derive(Resource)]
pub struct Grid<T> {
    pub entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
    _marker: PhantomData<T>,
}

#[derive(Resource)]
pub struct ConnectedComponents<T> {
    pub components: Vec<HashSet<GridLocation>>,
    _marker: PhantomData<T>,
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug, Deref, DerefMut)]
pub struct GridLocation(pub IVec2);

#[derive(Component)]
pub struct LockToGrid;

#[derive(Event)]
pub struct DirtyGridEvent<T>(pub GridLocation, PhantomData<T>);

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T: Component> Plugin for GridPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<T>>()
            .init_resource::<ConnectedComponents<T>>()
            .add_systems(
                Update,
                (lock_to_grid::<T>, update_connected_components::<T>),
            )
            .add_event::<DirtyGridEvent<T>>()
            // TODO move_on_grid / GridLocation change detection
            .add_systems(Startup, first_dirty_event::<T>)
            .add_systems(
                PreUpdate,
                (
                    add_to_grid::<T>,
                    remove_from_grid::<T>,
                    resolve_connected_components::<T>,
                ),
            );
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

// Forces some sane initializations of connected components
fn first_dirty_event<T: Component>(mut dirty: EventWriter<DirtyGridEvent<T>>) {
    dirty.send(DirtyGridEvent::<T>(
        GridLocation::new(0, 0),
        PhantomData::default(),
    ));
}

#[derive(Component)]
struct ConnectedTask<T> {
    task: Task<ConnectedComponents<T>>,
}

fn resolve_connected_components<T: Component>(
    mut commands: Commands,
    mut connected: ResMut<ConnectedComponents<T>>,
    // Should maybe be a resource?
    mut tasks: Query<(Entity, &mut ConnectedTask<T>)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.task)) {
            //TODO is there a way to make bevy auto remove these or not panic or something
            commands.entity(task_entity).despawn_recursive();
            *connected = result;
        }
    }
}

fn update_connected_components<T: Component>(
    mut commands: Commands,
    grid: Res<Grid<T>>,
    mut events: EventReader<DirtyGridEvent<T>>,
    // Should maybe be a resource?
    current_tasks: Query<Entity, With<ConnectedTask<T>>>,
) {
    if !events.is_empty() {
        events.clear();
        for task in &current_tasks {
            commands.entity(task).despawn_recursive();
        }

        let thread_pool = AsyncComputeTaskPool::get();
        let grid = Box::new(grid.clone());

        let task = thread_pool.spawn(async move {
            let starts = all_points()
                .into_iter()
                .filter(|point| !grid.occupied(point))
                .collect::<Vec<_>>();

            ConnectedComponents::<T> {
                components: connected_components::connected_components(&starts, |p| {
                    neumann_neighbors(&grid, p)
                }),
                ..default()
            }
        });

        commands.spawn(ConnectedTask { task });
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

fn all_points() -> Vec<GridLocation> {
    (0..GRID_SIZE)
        .flat_map(|x| (0..GRID_SIZE).map(move |y| GridLocation::new(x as u32, y as u32)))
        .collect()
}

impl<T> Default for ConnectedComponents<T> {
    fn default() -> Self {
        Self {
            components: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<T> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            entities: self.entities,
            _marker: self._marker,
        }
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

impl<T> ConnectedComponents<T> {
    pub fn point_to_component(&self, start: &GridLocation) -> Option<&HashSet<GridLocation>> {
        self.components
            .iter()
            .find(|component| component.contains(start))
    }

    pub fn in_same_component(&self, start: &GridLocation, end: &GridLocation) -> bool {
        self.point_to_component(start) == self.point_to_component(end)
    }

    pub fn random_point_in_same_component<R>(
        &self,
        start: &GridLocation,
        rng: &mut R,
    ) -> Option<GridLocation>
    where
        R: Rng + ?Sized,
    {
        self.point_to_component(start)
            .and_then(|component| component.iter().choose(rng).cloned())
    }
}
