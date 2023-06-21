use bevy::window::PresentMode;
use logic_management_tutorial::prelude::*;

pub const WIDTH: f32 = 1920.0;
pub const HEIGHT: f32 = 1080.0;

fn use_grid(
    grid: Res<Grid<Wall>>,
    walls: Query<&Wall>,
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, _) in grid.iter() {
        let _wall = walls
            .get(entity)
            .expect("entity in grid does not have wall component");
        if keyboard.just_pressed(KeyCode::A) {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_pawns(mut commands: Commands) {
    for _i in 0..1 {
        info!("Spawning");
        commands.spawn((
            SpatialBundle::default(),
            CharacterSprite::Stand(Facing::Down),
            Pawn,
            Brain::default(),
            AiPath::default(),
            Hunger { value: 100.0 },
            Recreation { value: 100.0 },
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Immediate,
                        title: "Logic Management Game".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(GraphicsPlugin)
        .add_plugin(AiPlugin)
        .add_plugin(SimpleCameraPlugin)
        .add_plugin(BuildingPlugin)
        .add_plugin(NeedsPlugin)
        .add_plugin(PathfindingPlugin)
        .add_systems(Update, use_grid)
        .add_systems(Startup, (spawn_walls, spawn_pawns))
        .run();
}

fn spawn_walls(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn((
            SpatialBundle::default(),
            Wall { _health: 10.0 },
            LockToGrid,
            WallSprite::Neutral,
            GridLocation::new(3, i),
        ));
    }
}
