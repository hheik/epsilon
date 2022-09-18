use std::path::Path;

use bevy::{
    asset::{AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::{types::*, Hull};

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
        unlit: true,
        ..default()
    })
    .with_dependency(base_color_path);

    load_context.set_labeled_asset(&path, material)
}
