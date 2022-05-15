use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::animation::AnimationTimer;
use crate::spritesheets::GabeSheet;
use crate::spritesheets::ManiSheet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system)
            .add_system(move_player_system);
    }
}

#[derive(Component, Inspectable)]
pub(crate) struct Player;

#[derive(Component, Inspectable)]
pub(crate) struct Follower;

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

fn spawn_player_system(
    mut commands: Commands,
    gabe_sheet: Res<GabeSheet>,
    mani_sheet: Res<ManiSheet>,
) {
    /*
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-200.0, 0.0, 1.0),
            sprite: Sprite {
                color: Color::rgb(0.3, 0.5, 0.5),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Direction::Right);
    */
    let gabe_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: gabe_sheet.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 12.0 * 4.0, 1.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Direction::Right)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .id();

    let mani_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: mani_sheet.0.clone(),
            transform: Transform {
                scale: Vec3::splat(1.0),
                translation: Vec3::new(-30.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Follower)
        .insert(Direction::Right)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .id();

    commands.entity(gabe_entity).push_children(&[mani_entity]);
}

// TODO: Put this in a plugin. A component for movable should be create
fn move_player_system(
    mut query: Query<(&mut Transform, &mut Direction), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let (mut player_transform, mut player_direction) = query.get_single_mut().unwrap();
    if input.pressed(KeyCode::Right) {
        *player_direction = Direction::Right;
        player_transform.translation.x += 1.0;
    }
    if input.pressed(KeyCode::Left) {
        *player_direction = Direction::Left;
        player_transform.translation.x -= 1.0
    }
    if input.pressed(KeyCode::Space) {
        player_transform.translation.y += 1.0;
    }
    if input.pressed(KeyCode::R) {
        player_transform.translation.x = 0.0;
        player_transform.translation.y = 0.0;
    }
}
