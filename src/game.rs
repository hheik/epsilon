use bevy::{input::mouse::MouseMotion, prelude::*, render::mesh::VertexAttributeValues, core::CorePlugin};

use crate::{
    q_map::{QMap, QMapAsset, QMapPlugin},
    util::math::*,
};

pub fn init() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(QMapPlugin)
        .add_startup_system(map_setup)
        // .add_startup_system(setup)
        // .add_system(camera_orbit)
        .run();
}

fn map_setup(
    asset_server: Res<AssetServer>
) {
    let q_map: Handle<QMapAsset> = asset_server.load("levels/station.map");
    // match q_map.get_field::<QMap>("q_map") {
    //     Some(some) => println!("something?"),
    //     None => println!("no q_map :("),
    // }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::Hsla {
            hue: 0.0,
            saturation: 0.0,
            lightness: 1.0,
            alpha: 1.0,
        },
        brightness: 1.0,
    });

    // // Scene
    // commands.spawn_bundle(SceneBundle {
    //     scene: asset_server.load("scenes/test_scene.gltf#Scene0"),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    // // Box
    // {
    //     let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    //     if let Some(VertexAttributeValues::Float32x3(positions)) =
    //         mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    //     {
    //         let colors: Vec<[f32; 4]> = positions
    //             .iter()
    //             .map(|[x, y, z]| {
    //                 let r = inverse_lerp(-0.5, 0.5, *x);
    //                 let g = inverse_lerp(-0.5, 0.5, *y);
    //                 let b = inverse_lerp(-0.5, 0.5, *z);
    //                 [r, g, b, 1.0]
    //             })
    //             .collect();
    //         mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    //     }
    //     commands.spawn_bundle(PbrBundle {
    //         mesh: meshes.add(mesh),
    //         material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
    //         transform: Transform::from_xyz(-1.5, 0.5, 0.0),
    //         ..default()
    //     });
    // }
}

fn camera_orbit(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let origin = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    for event in mouse_motion_events.iter() {
        for mut transform in query.iter_mut() {
            transform.rotate_around(origin, Quat::from_rotation_y(event.delta.x * -0.005));
            let right = transform.right();
            transform.rotate_around(
                origin,
                Quat::from_scaled_axis(right * event.delta.y * -0.005),
            )
        }
    }
}
