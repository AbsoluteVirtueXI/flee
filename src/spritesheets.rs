use bevy::prelude::*;
pub struct SpriteSheetsPlugin;

impl Plugin for SpriteSheetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheets);
    }
}

pub struct GabeSheet(pub Handle<TextureAtlas>);
pub struct ManiSheet(pub Handle<TextureAtlas>);

pub fn load_spritesheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_gabe_handle = asset_server.load("textures/gabe-idle-run.png");
    let texture_mani_handle = asset_server.load("textures/mani-idle-run.png");
    let texture_atlas_gabe =
        TextureAtlas::from_grid(texture_gabe_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_mani =
        TextureAtlas::from_grid(texture_mani_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_gabe_handle = texture_atlases.add(texture_atlas_gabe);
    let texture_atlas_mani_handle = texture_atlases.add(texture_atlas_mani);
    commands.insert_resource(GabeSheet(texture_atlas_gabe_handle));
    commands.insert_resource(ManiSheet(texture_atlas_mani_handle));
}
