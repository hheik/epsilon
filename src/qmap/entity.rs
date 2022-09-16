use bevy::{
    prelude::*,
    asset::{LoadContext, LoadedAsset},
    render::mesh::{PrimitiveTopology, Indices}
};

use super::types::*;

pub fn build_brush_entity<'a>(world: &mut World, load_context: &'a mut LoadContext, faces: Vec<Face>) {
    let origin = faces.first().unwrap().vertices.first().unwrap().position;
    for face in faces {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let material = StandardMaterial { ..default() };

        let (positions, uvs, normals) = face.as_tuples();
        // let (positions, uvs, normals): (Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f32; 3]>) = (
        //     vec![positions[0], positions[1], positions[2], positions[0], positions[2], positions[3]],
        //     vec![uvs[0], uvs[1], uvs[2], uvs[0], uvs[2], uvs[3]],
        //     vec![normals[0], normals[1], normals[2], normals[0], normals[2], normals[3]],
        // );

        let tri_count = positions.len() - 2;
        let mut indices: Vec<u16> = Vec::with_capacity(tri_count * 3);
        for i in 0..tri_count {
            indices.push(0);
            indices.push(i as u16 + 1);
            indices.push(i as u16 + 2);
        }
        for i in 0..tri_count {
            indices.push(0);
            indices.push(i as u16 + 2);
            indices.push(i as u16 + 1);
        }

        println!("{:?}", positions);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        mesh.set_indices(Some(Indices::U16(indices)));

        let mesh = load_context.set_labeled_asset(&"Mesh0", LoadedAsset::new(mesh));
        let material = load_context.set_labeled_asset(&"Material0", LoadedAsset::new(material));
        world.spawn().insert_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            // transform: Transform::from_xyz(origin.x, origin.y, origin.z),
            ..default()
        });
    }
    
    // world.spawn().insert_bundle(PbrBundle {
    //     mesh,
    //     material,
    //     transform: Transform::from_xyz(origin.x, origin.y, origin.z),
    //     ..default()
    // });
}
