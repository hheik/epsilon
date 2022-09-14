use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;
use shalrath::repr::Map;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "5ebe8fba-89af-4d06-8a2a-5d5184f2200a"]
pub struct QMapAsset {
    pub q_map: Map,
}

#[derive(Default)]
pub struct QMapAssetLoader;

impl AssetLoader for QMapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let q_map = String::from_utf8(bytes.to_vec())
                .expect("Failed to parse map as utf-8")
                .parse::<Map>()
                .expect("Failed to parse map");
            for entity in q_map.0.iter() {
                println!("entity:");
                for prop in entity.properties.iter() {
                    println!("    property {}: {}", prop.key, prop.value);
                };
                for brush in entity.brushes.iter() {
                    println!("    brush");
                    for plane in brush.0.iter() {
                        println!("        plane: {}", plane);
                    }
                }
            }
            load_context.set_default_asset(LoadedAsset::new(QMapAsset { q_map }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}
