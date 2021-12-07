use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 8.0, tile_size_y = 8.0, rows = 10, columns = 14))]
    #[asset(path = "textures/colored_tilemap_packed.png")]
    pub colored_tilemap_packed: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Kenney Mini.ttf")]
    pub mini: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/bookOpen.ogg")]
    pub book_open: Handle<AudioSource>,
    #[asset(path = "audio/clothBelt.ogg")]
    pub cloth_belt: Handle<AudioSource>,
    #[asset(path = "audio/doorOpen_2.ogg")]
    pub door_open: Handle<AudioSource>,
    #[asset(path = "audio/footstep04.ogg")]
    pub footstep: Handle<AudioSource>,
    #[asset(path = "audio/handleCoins.ogg")]
    pub handle_coins: Handle<AudioSource>,
    #[asset(path = "audio/knifeSlice.ogg")]
    pub knife_slice: Handle<AudioSource>,
    #[asset(path = "audio/metalClick.ogg")]
    pub metal_click: Handle<AudioSource>,
}
