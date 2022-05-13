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

fn spawn_player_system(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.5, 0.5),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn move_player_system(mut query: Query<&mut Transform, With<Player>>, input: Res<Input<KeyCode>>) {
    let mut player = query.get_single_mut().unwrap();
    if input.pressed(KeyCode::Right) {
        player.translation.x += 1.0;
    }
    if input.pressed(KeyCode::Left) {
        player.translation.x -= 1.0;
    }
    if input.pressed(KeyCode::Space) {
        player.translation.y += 1.0;
    }
    if input.pressed(KeyCode::R) {
        player.translation.x = 0.0;
        player.translation.y = 0.0;
    }
}
