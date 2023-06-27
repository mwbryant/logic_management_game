use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, click_to_build_walls);
    }
}

fn click_to_build_walls(
    mut commands: Commands,
    mut wall_grid: ResMut<Grid<Wall>>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(location) = GridLocation::from_world(cursor_position.world_position) {
            commands.spawn((
                SpatialBundle::default(),
                Wall { _health: 10.0 },
                LockToGrid,
                WallSprite::None,
                location,
            ));
        }
    }
}
