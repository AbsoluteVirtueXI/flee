use crate::spritesheets::GroundSheet;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground);
    }
}

#[derive(Component, Inspectable)]
pub struct Ground;

fn spawn_ground(mut commands: Commands, ground_sheet: Res<GroundSheet>) {
    for index in 20..=29 {
        let sprite = TextureAtlasSprite::new(index);

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: ground_sheet.0.clone(),
                transform: Transform {
                    translation: Vec3::new(-122.0 + (index as f32 - 20.0) * 16.0, -8.0, 1.0),
                    scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Ground);
    }
}
