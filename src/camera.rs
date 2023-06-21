use crate::prelude::*;
use bevy::{input::mouse::MouseWheel, render::camera::ScalingMode};

pub struct SimpleCameraPlugin;

impl Plugin for SimpleCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (camera_pan, camera_zoom));
    }
}

#[derive(Component)]
pub struct CameraSettings {
    pub pan_speed: f32,
    pub zoom_speed: f32,
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x = GRID_SIZE as f32 / 2.0;
    camera.transform.translation.y = GRID_SIZE as f32 / 2.0;

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 64.0,
        min_height: 36.0,
    };

    commands.spawn((
        camera,
        CameraSettings {
            pan_speed: 6.0,
            zoom_speed: 10.0,
        },
    ));
}

fn camera_zoom(
    mut camera: Query<(&mut OrthographicProjection, &CameraSettings), With<Camera2d>>,
    mut mouse: EventReader<MouseWheel>,
) {
    let (mut projection, settings) = camera.single_mut();
    if let ScalingMode::AutoMin {
        min_width,
        min_height,
    } = projection.scaling_mode
    {
        for ev in mouse.iter() {
            let width_delta = ev.y * settings.zoom_speed;
            let height_delta = width_delta * min_height / min_width;
            let new_width = num::clamp(min_width + width_delta, 16.0, 128.0);
            let new_height = num::clamp(min_height + height_delta, 9.0, 72.0);
            // TODO units
            projection.scaling_mode = ScalingMode::AutoMin {
                min_width: new_width,
                min_height: new_height,
            };
        }
    }
}

fn camera_pan(
    mut camera: Query<(&mut Transform, &CameraSettings), With<Camera2d>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, settings) = camera.single_mut();

    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds() * settings.pan_speed;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * settings.pan_speed;
    }
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * settings.pan_speed;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * settings.pan_speed;
    }
}
