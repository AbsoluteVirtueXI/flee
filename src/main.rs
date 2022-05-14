use bevy::prelude::*;
use flee::AnimationPlugin;
use flee::CameraPlugin;
use flee::DebugPlugin;
use flee::PlayerPlugin;
use flee::SpriteSheetsPlugin;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(SpriteSheetsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimationPlugin)
        .run();
}
