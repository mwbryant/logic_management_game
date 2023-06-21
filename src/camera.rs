use crate::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct SimpleCameraPlugin;

impl Plugin for SimpleCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_pan);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 64.0,
        min_height: 36.0,
    };

    commands.spawn(camera);
}

fn camera_pan(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = camera.single_mut();

    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds();
    }
}
