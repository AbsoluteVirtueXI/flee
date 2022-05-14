use crate::player::{Follower, Player};
use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation_system); // TODO should be chain after a movable component
    }
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

// Todo: Find a better query. Should be done in 1 query if possible.
fn animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
    mut follower_query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        (With<Follower>, Without<Player>),
    >,
) {
    let (mut timer, mut sprite, texture_atlas_handle) = player_query.get_single_mut().unwrap();
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
    }

    let (mut timer, mut sprite, texture_atlas_handle) = follower_query.get_single_mut().unwrap();
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
    }
}
