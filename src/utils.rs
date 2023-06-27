// General utility functions, nothing interesting
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2,
    pub over_ui: bool,
}

pub fn update_cursor(
    mut cursor: ResMut<CursorPosition>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, transform) = camera.single();

    if let Some(screen_position) = window.cursor_position() {
        let world_position = camera
            .viewport_to_world(transform, screen_position)
            .unwrap()
            .origin
            .truncate();
        cursor.world_position = world_position;
    }
}
