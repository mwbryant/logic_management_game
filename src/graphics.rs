use std::ops::Deref;

use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WalkCycle {
    Stand(bool),
    #[default]
    StepLeftFoot,
    StepRightFoot,
}

impl WalkCycle {
    pub fn next_frame(&self) -> Self {
        match self {
            WalkCycle::Stand(last) => {
                if *last {
                    WalkCycle::StepLeftFoot
                } else {
                    WalkCycle::StepRightFoot
                }
            }
            WalkCycle::StepLeftFoot => WalkCycle::Stand(false),
            WalkCycle::StepRightFoot => WalkCycle::Stand(true),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct CharacterSprite {
    pub walk_stage: WalkCycle,
    pub facing: Facing,
}

impl CharacterSprite {
    pub fn next_frame(&mut self) {
        self.walk_stage = self.walk_stage.next_frame();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum WallSprite {
    All,
    None,
    NorthSouth,
    EastWest,
    East,
    South,
    West,
    North,
    EastSouth,
    WestSouth,
    NorthWest,
    NorthEast,
    NorthWestSouth,
    NorthEastWest,
    NorthEastSouth,
    EastWestSouth,
    Outline,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub enum Facing {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl Facing {
    //Gross but works ugh
    pub fn from_direction(direction: &Vec2) -> Self {
        if direction.x == 0.0 && direction.y == 0.0 {
            Facing::Down
        } else if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            }
        } else if direction.y > 0.0 {
            Facing::Up
        } else {
            Facing::Down
        }
    }
}

impl IndexableSprite for WallSprite {
    type AtlasHandleWrapper = CharacterAtlas;
    fn index(&self) -> usize {
        match self {
            WallSprite::All => 12,
            WallSprite::None => 13,
            WallSprite::NorthSouth => 14,
            WallSprite::EastWest => 15,

            WallSprite::East => 12 + 16,
            WallSprite::South => 13 + 16,
            WallSprite::West => 14 + 16,
            WallSprite::North => 15 + 16,

            WallSprite::EastSouth => 12 + 16 * 2,
            WallSprite::WestSouth => 13 + 16 * 2,
            WallSprite::NorthWest => 14 + 16 * 2,
            WallSprite::NorthEast => 15 + 16 * 2,

            WallSprite::NorthWestSouth => 12 + 16 * 3,
            WallSprite::NorthEastWest => 13 + 16 * 3,
            WallSprite::NorthEastSouth => 14 + 16 * 3,
            WallSprite::EastWestSouth => 15 + 16 * 3,

            WallSprite::Outline => 15 + 16 * 4,
        }
    }
}

impl IndexableSprite for CharacterSprite {
    type AtlasHandleWrapper = CharacterAtlas;
    fn index(&self) -> usize {
        self.walk_stage.index() + 16 * self.facing.index()
    }
}

impl WalkCycle {
    fn index(&self) -> usize {
        match self {
            WalkCycle::Stand(_) => 0,
            WalkCycle::StepLeftFoot => 1,
            WalkCycle::StepRightFoot => 2,
        }
    }
}

impl Facing {
    fn index(&self) -> usize {
        match self {
            Facing::Down => 0,
            Facing::Up => 1,
            Facing::Right => 2,
            Facing::Left => 3,
        }
    }
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterAtlas>()
            .add_systems(
                Update,
                (
                    update_indexable_sprite::<CharacterSprite>,
                    update_indexable_sprite::<WallSprite>,
                    update_wall_sprite,
                ),
            )
            .add_systems(
                PreUpdate,
                (
                    add_sprite_to_indexable::<CharacterSprite>,
                    add_sprite_to_indexable::<WallSprite>,
                ),
            );
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct CharacterAtlas(Handle<TextureAtlas>);

impl FromWorld for CharacterAtlas {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<AssetServer>().unwrap();
        let texture_handle = assets.load("characters.png");

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        CharacterAtlas(texture_atlas_handle)
    }
}

fn add_sprite_to_indexable<T: Component + IndexableSprite>(
    mut commands: Commands,
    sprites: Query<Entity, (Added<T>, Without<TextureAtlasSprite>)>,
    atlas: Res<T::AtlasHandleWrapper>,
) {
    for character in &sprites {
        let handle = atlas.deref().deref();
        commands.entity(character).insert((
            handle.clone(),
            TextureAtlasSprite {
                custom_size: Some(Vec2::ONE),
                ..default()
            },
        ));
    }
}

fn update_indexable_sprite<T: Component + IndexableSprite>(
    mut sprites: Query<(&T, &mut TextureAtlasSprite)>,
) {
    for (indexable, mut sprite) in sprites.iter_mut() {
        sprite.index = indexable.index();
    }
}

fn update_wall_sprite(mut sprites: Query<&mut WallSprite>, grid: Res<Grid<Wall>>) {
    for (ent, location) in grid.iter() {
        let mut wall = sprites.get_mut(ent).expect("Wall with no sprite in grid");

        let east = &(location.0 + IVec2::new(1, 0)).into();
        let west = &(location.0 - IVec2::new(1, 0)).into();
        let north = &(location.0 + IVec2::new(0, 1)).into();
        let south = &(location.0 - IVec2::new(0, 1)).into();

        use WallSprite::*;
        *wall = match (
            grid.occupied(west),
            grid.occupied(east),
            grid.occupied(north),
            grid.occupied(south),
        ) {
            (true, true, true, true) => All,
            (true, true, true, false) => NorthEastWest,
            (true, true, false, true) => EastWestSouth,
            (true, true, false, false) => EastWest,
            (true, false, true, true) => NorthWestSouth,
            (true, false, true, false) => NorthWest,
            (true, false, false, true) => WestSouth,
            (true, false, false, false) => West,
            (false, true, true, true) => NorthEastSouth,
            (false, true, true, false) => NorthEast,
            (false, true, false, true) => EastSouth,
            (false, true, false, false) => East,
            (false, false, true, true) => NorthSouth,
            (false, false, true, false) => North,
            (false, false, false, true) => South,
            (false, false, false, false) => None,
        };
    }
}

pub trait IndexableSprite {
    type AtlasHandleWrapper: Resource + std::ops::Deref<Target = Handle<TextureAtlas>>;

    fn index(&self) -> usize;
}
