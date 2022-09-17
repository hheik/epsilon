use self::loader::QMapLoader;
use bevy::prelude::*;

mod entity;
mod loader;
mod types;

pub struct QMapPlugin;

impl Plugin for QMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset_loader::<QMapLoader>();
    }
}
