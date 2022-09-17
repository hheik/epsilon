use bevy::prelude::*;

use self::image_import::ImageImportPlugin;

mod image_import;

pub struct ImporterPlugins;

impl PluginGroup for ImporterPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(ImageImportPlugin);
    }
}
