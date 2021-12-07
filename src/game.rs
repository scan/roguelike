use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;

use crate::{
    assets::{AudioAssets, FontAssets, TextureAssets},
    state::GameState,
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
    }
}
