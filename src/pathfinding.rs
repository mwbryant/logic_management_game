use std::error::Error;

use pathfinding::prelude::astar;

use crate::grid::{Grid, GridLocation, GRID_SIZE};

// TODO this grid location is tied to a grid
fn neumann_neighbors<T>(grid: &Grid<T>, location: &GridLocation) -> Vec<(GridLocation, usize)> {
    let (x, y) = (location.x, location.y);

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

// OPT precalculate sucessors? Look into pathfinding::grid
impl GridLocation {
    fn distance(&self, other: &GridLocation) -> usize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as usize
    }

    pub fn path_to<T>(
        &self,
        goal: &GridLocation,
        grid: &Grid<T>,
    ) -> Result<Vec<GridLocation>, PathfindingError> {
        let result = astar(
            self,
            |p| neumann_neighbors(grid, p),
            |p| p.distance(goal) / 3,
            |p| p == goal,
        );

        if let Some((path, _length)) = result {
            Ok(path)
        } else {
            Err(PathfindingError)
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

        let result = start.path_to(&goal, &grid);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
