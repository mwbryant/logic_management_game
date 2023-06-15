use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum CharacterSprite {
    Stand(Facing),
    StepLeftFoot(Facing),
    StepRightFoot(Facing),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl CharacterSprite {
    pub fn to_index(&self) -> usize {
        match self {
            CharacterSprite::Stand(direction) => 0 + direction.to_index() * 16,
            CharacterSprite::StepLeftFoot(direction) => 1 + direction.to_index() * 16,
            CharacterSprite::StepRightFoot(direction) => 2 + direction.to_index() * 16,
        }
    }
}

impl Facing {
    pub fn to_index(&self) -> usize {
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
            .add_systems(Update, update_character_sprite)
            .add_systems(PreUpdate, add_sprite_to_character);
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

fn add_sprite_to_character(
    mut commands: Commands,
    characters: Query<Entity, (Added<CharacterSprite>, Without<TextureAtlasSprite>)>,
    character_atlas: Res<CharacterAtlas>,
) {
    for character in &characters {
        commands.entity(character).insert((
            character_atlas.0.clone(),
            TextureAtlasSprite {
                custom_size: Some(Vec2::ONE),
                ..default()
            },
        ));
    }
}

fn update_character_sprite(mut characters: Query<(&CharacterSprite, &mut TextureAtlasSprite)>) {
    for (character_sprite, mut sprite) in characters.iter_mut() {
        sprite.index = character_sprite.to_index();
    }
}
