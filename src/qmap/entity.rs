use std::path::Path;

use bevy::{
    asset::{AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    utils::HashMap,
};

use super::{loader::convert_coords, types::*, Hull};

pub fn build_brush_entity<'a>(
    world: &mut World,
    load_context: &'a mut LoadContext,
    mesh_counter: &mut u16,
    faces: Vec<Face>,
) {
    let origin = faces.first().unwrap().vertices.first().unwrap().position;
    let mut hull: Vec<Vec3> = vec![];

    let mut children: Vec<Entity> = vec![];

    for face in faces.iter().map(|face| face.offset_to_origin(origin)) {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let (positions, uvs, normals) = face.as_tuples();

        let tri_count = positions.len() - 2;
        let mut indices: Vec<u16> = Vec::with_capacity(tri_count * 3);
        for i in 0..tri_count {
            indices.push(0);
            indices.push(i as u16 + 1);
            indices.push(i as u16 + 2);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        mesh.set_indices(Some(Indices::U16(indices)));
        mesh.generate_tangents()
            .expect("Could not generate tangents for brush");

        let mesh =
            load_context.set_labeled_asset(&format!("mesh/{mesh_counter}"), LoadedAsset::new(mesh));
        let material = load_material(load_context, format!("textures/{}", face.texture));

        children.push(
            world
                .spawn()
                .insert_bundle(PbrBundle {
                    mesh,
                    material,
                    ..default()
                })
                .id(),
        );

        for vertex in face.vertices.iter() {
            // Add vertex to hull if it doesn't already exist
            if !hull
                .iter()
                .any(|point| point.abs_diff_eq(vertex.position, 0.01))
            {
                hull.push(vertex.position);
            }
        }

        *mesh_counter += 1;
    }

    world
        .spawn()
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            origin.x, origin.y, origin.z,
        )))
        .insert_bundle(VisibilityBundle::default())
        .insert(Hull { points: hull })
        .push_children(&children);
}

fn load_material<'a>(load_context: &'a mut LoadContext, path: String) -> Handle<StandardMaterial> {
    let base_color_path = format!("{}.png", path.as_str().to_owned());
    let base_color_path = AssetPath::new_ref(Path::new(&base_color_path), None);
    let base_color_texture: Option<Handle<Image>> =
        Some(load_context.get_handle(base_color_path.clone()));

    let material = LoadedAsset::new(StandardMaterial {
        base_color: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        },
        base_color_texture,
        metallic: 0.0,
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        unlit: false,
        ..default()
    })
    .with_dependency(base_color_path);

    load_context.set_labeled_asset(&path, material)
}

pub fn build_entity(world: &mut World, class_name: String, prop_map: HashMap<String, String>) {
    match class_name.as_str() {
        "light_point" => {
            let default_origin = &"0 0 0".to_string();
            let origin: Vec<&str> = prop_map
                .get("origin")
                .unwrap_or(default_origin)
                .split_ascii_whitespace()
                .collect();
            let origin = if origin.len() == 3 {
                Vec3 {
                    x: origin[0].parse::<f32>().unwrap_or_default(),
                    y: origin[1].parse::<f32>().unwrap_or_default(),
                    z: origin[2].parse::<f32>().unwrap_or_default(),
                }
            } else {
                Vec3::ZERO
            };
            let origin = convert_coords(origin);
            world.spawn().insert_bundle(PointLightBundle {
                point_light: PointLight {
                    intensity: prop_map
                        .get("intensity")
                        .unwrap_or(&"800.0".to_string())
                        .parse::<f32>()
                        .unwrap_or_default(),
                    range: prop_map
                        .get("range")
                        .unwrap_or(&"8.0".to_string())
                        .parse::<f32>()
                        .unwrap_or_default(),
                    color: Color::hsl(0.50, 0.15, 0.7),
                    // color: Color::WHITE,
                    ..default()
                },
                transform: Transform::from_xyz(origin.x, origin.y, origin.z),
                ..default()
            });
        }
        _ => (),
    };
}
