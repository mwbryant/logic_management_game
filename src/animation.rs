use crate::prelude::*;

pub struct FrameAnimationPlugin;

impl Plugin for FrameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_character_sprite_animation);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct LastDirection(pub Vec2);

#[derive(Component, Deref, DerefMut)]
//TODO private
pub struct AnimationTimer(pub Timer);

fn update_character_sprite_animation(
    mut sprites: Query<(&mut CharacterSprite, &LastDirection, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut sprite, direction, mut animation) in &mut sprites {
        animation.tick(time.delta());
        if animation.just_finished() {
            sprite.facing = Facing::from_direction(direction);
            sprite.next_frame();
        }
    }
}
