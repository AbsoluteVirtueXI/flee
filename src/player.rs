use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system)
            .add_system(move_player_system);
    }
}

#[derive(Component, Inspectable)]
pub(crate) struct Player;

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

fn spawn_player_system(mut commands: Commands) {
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
}

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
