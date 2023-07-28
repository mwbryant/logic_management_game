use crate::prelude::*;
use bevy::render::{
    render_resource::SpecializedRenderPipelines, renderer::RenderDevice, RenderApp,
};
use rand::Rng;

pub const WIDTH: f32 = 1920.0;
pub const HEIGHT: f32 = 1080.0;

fn save_game(entities: Query<EntityRef>) {}

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
        if keyboard.just_pressed(KeyCode::P) {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_pawns(mut commands: Commands) {
    for _i in 0..10 {
        let position = Vec2::new(GRID_SIZE as f32 / 2., GRID_SIZE as f32 / 2.);
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(position.extend(800.0))),
            CharacterSprite::default(),
            Pawn,
            LastDirection(Vec2::ZERO),
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            Brain::default(),
            AiPath::default(),
            Hunger { value: 100.0 },
            Recreation { value: 100.0 },
        ));
    }
}

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    //present_mode: PresentMode::Immediate,
                    title: "Logic Management Game".into(),
                    resolution: (WIDTH, HEIGHT).into(),
                    resizable: false,
                    //mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
            .build(),
    )
    .add_plugins((
        GraphicsPlugin,
        AiPlugin,
        SimpleCameraPlugin,
        FrameAnimationPlugin,
        BuildingPlugin,
        NeedsPlugin,
        PathfindingPlugin,
        PlayerPlugin,
    ))
    .init_resource::<CursorPosition>()
    .add_systems(Update, update_cursor)
    .add_systems(Update, use_grid)
    .add_systems(Startup, (spawn_pawns, spawn_outline));

    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.run();
}

fn spawn_outline(mut commands: Commands) {
    for i in 0..GRID_SIZE {
        spawn_outline_wall(&mut commands, i as f32, -1.0);
        spawn_outline_wall(&mut commands, i as f32, GRID_SIZE as f32);
    }
    for j in -1..(GRID_SIZE as i32 + 1) {
        spawn_outline_wall(&mut commands, -1.0, j as f32);
        spawn_outline_wall(&mut commands, GRID_SIZE as f32, j as f32);
    }
}

fn spawn_outline_wall(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
        WallSprite::Outline,
    ));
}

pub enum MazeTile {
    Wall,
    Open(usize),
}

#[allow(dead_code)]
fn spawn_maze(mut commands: Commands) {
    let mut maze = Grid::<MazeTile>::default();
    let mut rng = rand::thread_rng();
    for i in 0..GRID_SIZE as u32 {
        for j in 0..GRID_SIZE as u32 {
            let location = GridLocation::new(i, j);
            if rng.gen::<f32>() < 0.3 {
                //Ugh I hate having to do this to use my grid
                maze[&location] = Some(Entity::from_raw(0));
            }
        }
    }

    maze[&GridLocation::new(GRID_SIZE as u32 / 2, GRID_SIZE as u32 / 2)] = None;
    maze[&GridLocation::new(10, 10)] = None;
    maze[&GridLocation::new(10, 9)] = None;

    for (_, filled) in maze.iter() {
        commands.spawn((
            SpatialBundle::default(),
            Wall { _health: 10.0 },
            LockToGrid,
            WallSprite::None,
            filled,
        ));
    }

    commands.spawn((
        SpatialBundle::default(),
        Machine {
            use_offset: IVec2 { x: 0, y: -1 },
        },
        FoodMachine { rate: 10.0 },
        LockToGrid,
        MachineSprite::FoodMachine,
        Wall { _health: 10.0 },
        GridLocation::new(10, 10),
    ));
}
