use bevy::{
    asset::{LoadContext, LoadedAsset},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::types::*;

pub fn build_brush_entity<'a>(
    world: &mut World,
    load_context: &'a mut LoadContext,
    mesh_counter: &mut u16,
    faces: Vec<Face>,
) {
    let origin = faces.first().unwrap().vertices.first().unwrap().position;
    for face in faces.iter().map(|face| face.offset_to_origin(origin)) {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let material = StandardMaterial { ..default() };

        let (positions, uvs, normals) = face.as_tuples();

        let tri_count = positions.len() - 2;
        let mut indices: Vec<u16> = Vec::with_capacity(tri_count * 3);
        for i in 0..tri_count {
            indices.push(0);
            indices.push(i as u16 + 1);
            indices.push(i as u16 + 2);
        }

        println!("{:?}", positions);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        mesh.set_indices(Some(Indices::U16(indices)));

        let mesh =
            load_context.set_labeled_asset(&format!("Mesh{mesh_counter}"), LoadedAsset::new(mesh));
        let material = load_context
            .set_labeled_asset(&format!("mat_{}", face.texture), LoadedAsset::new(material));
        world.spawn().insert_bundle(PbrBundle {
            mesh,
            material,
            // transform: Transform::from_xyz(0.0, 0.0, 0.0),
            transform: Transform::from_xyz(origin.x, origin.y, origin.z),
            ..default()
        });
        *mesh_counter += 1;
    }
}
