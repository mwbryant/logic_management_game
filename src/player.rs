use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (left_click_to_build_walls, right_click_to_remove_walls),
        );
    }
}

fn left_click_to_build_walls(
    mut commands: Commands,
    wall_grid: Res<Grid<Wall>>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<Input<MouseButton>>,
) {
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    if let Some(location) = GridLocation::from_world(cursor_position.world_position) {
        if wall_grid.occupied(&location) {
            return;
        }
        commands.spawn((
            SpatialBundle::default(),
            Wall { _health: 10.0 },
            LockToGrid,
            WallSprite::None,
            location,
        ));
    }
}

fn right_click_to_remove_walls(
    mut commands: Commands,
    wall_grid: Res<Grid<Wall>>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<Input<MouseButton>>,
) {
    if !mouse.pressed(MouseButton::Right) {
        return;
    }
    if let Some(location) = GridLocation::from_world(cursor_position.world_position) {
        if let Some(entity) = wall_grid[&location] {
            commands.entity(entity).despawn_recursive();
        }
    }
}
