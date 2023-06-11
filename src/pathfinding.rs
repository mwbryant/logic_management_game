use std::error::Error;

use pathfinding::prelude::astar;

use crate::grid::{Grid, GridLocation, GRID_SIZE};

// OPT precalculate sucessors? Look into pathfinding::grid
impl GridLocation {
    // TODO filter based on grid
    fn neumann_neighbors(&self) -> Vec<(GridLocation, usize)> {
        let (x, y) = self.position;

        let mut sucessors = Vec::new();
        if let Some(left) = x.checked_sub(1) {
            sucessors.push((GridLocation::new(left, y), 1));
        }
        if let Some(down) = y.checked_sub(1) {
            sucessors.push((GridLocation::new(x, down), 1));
        }
        if x + 1 < GRID_SIZE {
            let right = x + 1;
            sucessors.push((GridLocation::new(right, y), 1));
        }
        if y + 1 < GRID_SIZE {
            let up = y + 1;
            sucessors.push((GridLocation::new(x, up), 1));
        }
        sucessors
    }

    fn distance(&self, other: &GridLocation) -> usize {
        self.position.0.abs_diff(other.position.0) + self.position.1.abs_diff(other.position.1)
    }

    fn path_to(&self, goal: &GridLocation) -> Result<Vec<GridLocation>, PathfindingError> {
        let result = astar(
            self,
            |p| p.neumann_neighbors(),
            |p| p.distance(&goal) / 3,
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

    use pathfinding::prelude::astar;

    use crate::grid::GridLocation;

    #[test]
    fn basic_pathfinding() {
        let goal = GridLocation::new(4, 6);
        let start = GridLocation::new(1, 1);
        let result = start.path_to(&goal);
        assert!(result.is_ok());
    }
}
