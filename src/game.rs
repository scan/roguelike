use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_pixel_camera::PixelCameraBundle;

use crate::{
    assets::{AudioAssets, FontAssets, TextureAssets},
    state::GameState, component,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading);

        AssetLoader::new(GameState::Loading, GameState::Running)
            .with_collection::<TextureAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<FontAssets>()
            .build(app);

        app.add_system_set(
            SystemSet::on_enter(GameState::Running).with_system(load_camera.system()).with_system(spawn_player.system()),
        );
    }
}

fn load_camera(mut commands: Commands) {
    commands.spawn_bundle(PixelCameraBundle::from_resolution(240, 160));
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(5),
        texture_atlas: textures.colored_tilemap_packed.clone(),
        ..Default::default()
    }).insert(component::Player);
}
