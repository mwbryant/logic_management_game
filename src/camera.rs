use crate::prelude::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    render::camera::ScalingMode,
};

pub struct SimpleCameraPlugin;

impl Plugin for SimpleCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (camera_pan, camera_zoom));
    }
}

#[derive(Component)]
pub struct CameraSettings {
    pub speed: f32,
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 64.0,
        min_height: 36.0,
    };

    commands.spawn((camera, CameraSettings { speed: 3.0 }));
}

fn camera_zoom(
    mut camera: Query<(&mut OrthographicProjection, &CameraSettings), With<Camera2d>>,
    mut mouse: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let (mut projection, settings) = camera.single_mut();
    if let ScalingMode::AutoMin {
        min_width,
        min_height,
    } = projection.scaling_mode
    {
        for ev in mouse.iter() {
            let width_delta = ev.y * settings.speed * time.delta_seconds();
            let height_delta = width_delta * min_height / min_width;
            // TODO units
            projection.scaling_mode = ScalingMode::AutoMin {
                min_width: min_width + width_delta,
                min_height: min_height + height_delta,
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
        transform.translation.x -= time.delta_seconds() * settings.speed;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * settings.speed;
    }
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * settings.speed;
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * settings.speed;
    }
}
