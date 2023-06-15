use std::ops::Deref;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum CharacterSprite {
    Stand(Facing),
    StepLeftFoot(Facing),
    StepRightFoot(Facing),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum WallSprite {
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl IndexableSprite for WallSprite {
    type AtlasHandleWrapper = CharacterAtlas;
    fn index(&self) -> usize {
        match self {
            WallSprite::Neutral => 15,
        }
    }
}

impl IndexableSprite for CharacterSprite {
    type AtlasHandleWrapper = CharacterAtlas;
    fn index(&self) -> usize {
        match self {
            CharacterSprite::Stand(direction) => direction.index() * 16,
            CharacterSprite::StepLeftFoot(direction) => 1 + direction.index() * 16,
            CharacterSprite::StepRightFoot(direction) => 2 + direction.index() * 16,
        }
    }
}

impl Facing {
    fn index(&self) -> usize {
        match self {
            Facing::Down => 0,
            Facing::Up => 1,
            Facing::Left => 2,
            Facing::Right => 3,
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
