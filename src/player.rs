use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClickMode>().add_systems(
            Update,
            (left_click_to_build, right_click_to_remove, set_build_mode),
        );
    }
}

#[derive(Default, Resource)]
enum ClickMode {
    None,
    BuildWall,
    #[default]
    BuildFoodMachine,
}

fn set_build_mode(keyboard: Res<Input<KeyCode>>, mut mode: ResMut<ClickMode>) {
    if keyboard.just_pressed(KeyCode::Key1) {
        *mode = ClickMode::None;
    }
    if keyboard.just_pressed(KeyCode::Key2) {
        *mode = ClickMode::BuildWall;
    }
    if keyboard.just_pressed(KeyCode::Key3) {
        *mode = ClickMode::BuildFoodMachine;
    }
}

fn left_click_to_build(
    mut commands: Commands,
    wall_grid: Res<Grid<Wall>>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<Input<MouseButton>>,
    mode: Res<ClickMode>,
) {
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    if let Some(location) = GridLocation::from_world(cursor_position.world_position) {
        if wall_grid.occupied(&location) {
            return;
        }
        match mode.as_ref() {
            ClickMode::None => {}
            ClickMode::BuildWall => {
                commands.spawn((
                    SpatialBundle::default(),
                    Wall { _health: 10.0 },
                    LockToGrid,
                    WallSprite::None,
                    location,
                ));
            }
            ClickMode::BuildFoodMachine => {
                commands.spawn((
                    SpatialBundle::default(),
                    Machine {
                        use_offset: IVec2 { x: 0, y: -1 },
                    },
                    FoodMachine { rate: 10.0 },
                    LockToGrid,
                    MachineSprite::FoodMachine,
                    Wall { _health: 10.0 },
                    location,
                ));
            }
        }
    }
}

fn right_click_to_remove(
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
