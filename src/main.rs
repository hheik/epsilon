use bevy::{input::mouse::MouseMotion, prelude::*, render::mesh::VertexAttributeValues};
use util::math::*;

mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_orbit)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    // Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.25, 0.25, 0.2).into()),
        ..default()
    });

    // Box
    {
        let mesh = Mesh::from(shape::Cube { size: 1.0 });
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(1.5, 0.5, 0.0),
            ..default()
        });
    }

    // Box
    {
        let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[x, y, z]| {
                    let r = inverse_lerp(-0.5, 0.5, *x);
                    let g = inverse_lerp(-0.5, 0.5, *y);
                    let b = inverse_lerp(-0.5, 0.5, *z);
                    [r, g, b, 1.0]
                })
                .collect();
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        });
    }
}

fn camera_orbit(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for event in mouse_motion_events.iter() {
        for mut transform in query.iter_mut() {
            transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(event.delta.x * -0.005));
            let right = transform.right();
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_scaled_axis(right * event.delta.y * -0.005),
            )
        }
    }
}
