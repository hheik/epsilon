use self::loader::QMapAssetLoader;
use bevy::prelude::*;

mod loader;

pub use loader::QMapAsset;
pub use shalrath::repr::Map as QMap;

pub struct QMapPlugin;

impl Plugin for QMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset_loader::<QMapAssetLoader>();
    }
}
