use std::{f32::consts::PI, ops::Deref};

use bevy::prelude::*;

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
    Neutral,
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
            WallSprite::Neutral => 15,
            WallSprite::Outline => 14,
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

pub trait IndexableSprite {
    type AtlasHandleWrapper: Resource + std::ops::Deref<Target = Handle<TextureAtlas>>;

    fn index(&self) -> usize;
}
