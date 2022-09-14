use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    render::render_resource::PrimitiveTopology,
    utils::BoxedFuture,
};
use shalrath::repr::Map;

const INVERSE_SCALE_FACTOR: f32 = 16.0;
const MAP_SCALE: f32 = 1.0 / INVERSE_SCALE_FACTOR;

#[derive(Default)]
pub struct QMapLoader;

impl AssetLoader for QMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move { Ok(load_qmap(bytes, load_context, MAP_SCALE).await?) })
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}

async fn load_qmap<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
    map_scale: f32,
) -> Result<(), bevy::asset::Error> {
    let qmap = String::from_utf8(bytes.to_vec())
        .expect("Failed to parse map as utf-8")
        .parse::<Map>()
        .expect("Failed to parse map");

    let mut world = World::default();
    let mut meshes: Vec<Handle<Mesh>> = vec![];

    for entity in qmap.0.iter() {
        // println!("entity:");
        for prop in entity.properties.iter() {
            // println!("    property {}: {}", prop.key, prop.value);
        }
        for brush in entity.brushes.iter() {
            // println!("    brush");
            for plane in brush.0.iter() {
                // println!("        plane: {}", plane);
                let mesh = Mesh::from(shape::Cube { size: 1.0 });
                let mesh = load_context
                    .set_labeled_asset(&format!("Mesh{}", meshes.len()), LoadedAsset::new(mesh));
                world
                    .spawn()
                    .insert_bundle(SpatialBundle::visible_identity())
                    .insert_bundle(PbrBundle {
                        mesh,
                        // material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                        transform: Transform::from_xyz(
                            plane.plane.v0.x * map_scale,
                            plane.plane.v0.y * map_scale,
                            plane.plane.v0.z * map_scale,
                        ),
                        ..default()
                    });
            }
        }
    }

    // let scene_bundle = SceneBundle::from_world(&mut world);
    let scene = Scene::new(world);

    load_context.set_default_asset(LoadedAsset::new(scene));

    Ok(())
}

fn load_brush() {}
