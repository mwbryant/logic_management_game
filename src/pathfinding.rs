use std::collections::VecDeque;

use crate::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use pathfinding::prelude::astar;

use crate::grid::{Grid, GridLocation, GRID_SIZE};

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_pathfinding_to_ai);
    }
}

#[derive(Component, Default)]
pub struct AiPath {
    pub locations: VecDeque<Vec2>,
}

fn neumann_neighbors<T>(grid: &Grid<T>, location: &GridLocation) -> Vec<(GridLocation, usize)> {
    let (x, y) = (location.x as u32, location.y as u32);

    let mut sucessors = Vec::new();
    if let Some(left) = x.checked_sub(1) {
        let location = GridLocation::new(left, y);
        if !grid.occupied(&location) {
            sucessors.push((location, 1));
        }
    }
    if let Some(down) = y.checked_sub(1) {
        let location = GridLocation::new(x, down);
        if !grid.occupied(&location) {
            sucessors.push((location, 1));
        }
    }
    if x + 1 < GRID_SIZE as u32 {
        let right = x + 1;
        let location = GridLocation::new(right, y);
        if !grid.occupied(&location) {
            sucessors.push((location, 1));
        }
    }
    if y + 1 < GRID_SIZE as u32 {
        let up = y + 1;
        let location = GridLocation::new(x, up);
        if !grid.occupied(&location) {
            sucessors.push((location, 1));
        }
    }
    sucessors
}

pub struct Path {
    pub steps: Vec<GridLocation>,
}

impl Path {
    pub fn optimize_corners(&mut self) {
        // i must be tracked here because vec len changes
        let mut i = 0;
        while i + 2 < self.steps.len() {
            let first_step = &self.steps[i];
            let third_step = &self.steps[i + 2];
            //If both x and y change then this is a corner
            if first_step.x != third_step.x && first_step.y != third_step.y {
                self.steps.remove(i + 1);
            }
            i += 1;
        }
    }
}

// OPT precalculate sucessors? Look into pathfinding::grid
impl GridLocation {
    fn distance(&self, other: &GridLocation) -> usize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as usize
    }
}

impl<T> Grid<T> {
    pub fn path_to(
        &self,
        start: &GridLocation,
        goal: &GridLocation,
    ) -> Result<Path, PathfindingError> {
        let result = astar(
            start,
            |p| neumann_neighbors(self, p),
            |p| p.distance(goal) / 3,
            |p| p == goal,
        );

        if let Some((steps, _length)) = result {
            Ok(Path { steps })
        } else {
            Err(PathfindingError)
        }
    }
}

#[derive(Component)]
pub struct PathfindingTask(Task<Result<Path, PathfindingError>>);

pub fn spawn_optimized_pathfinding_task<T: Component>(
    commands: &mut Commands,
    target: Entity,
    grid: &Grid<T>,
    start: GridLocation,
    end: GridLocation,
) {
    // Fail early if end is not valid
    if grid.occupied(&end) {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();

    // Must clone because the grid can change between frames
    // Must box to prevent stack overflows on very large grids
    let grid = Box::new(grid.clone());

    let task = thread_pool.spawn(async move {
        let mut path = grid.path_to(&start, &end);
        let _ = path.as_mut().map(|p| p.optimize_corners());
        path
    });

    commands.entity(target).insert(PathfindingTask(task));
}

pub fn apply_pathfinding_to_ai(
    mut commands: Commands,
    mut paths: Query<&mut AiPath>,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(task_entity).remove::<PathfindingTask>();

            if let Ok(mut ai_path) = paths.get_mut(task_entity) {
                if let Ok(path) = result {
                    ai_path.locations.clear();
                    for location in path.steps.iter() {
                        ai_path
                            .locations
                            .push_back(Vec2::new(location.x as f32, location.y as f32));
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PathfindingError;

#[cfg(test)]
mod tests {

    use bevy::prelude::Entity;

    use crate::grid::{Grid, GridLocation};

    #[test]
    fn basic_pathfinding() {
        let goal = GridLocation::new(4, 6);
        let start = GridLocation::new(1, 1);
        let mut grid: Grid<()> = Grid::default();
        grid.entities[2][0] = Some(Entity::from_raw(0));
        grid.entities[2][1] = Some(Entity::from_raw(0));
        grid.entities[2][2] = Some(Entity::from_raw(0));

        let result = grid.path_to(&start, &goal);
        assert!(result.is_ok());
    }
}
