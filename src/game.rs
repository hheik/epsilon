use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{import::ImporterPlugins, qmap::QMapPlugin};

pub fn init() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ImporterPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(QMapPlugin)
        .add_startup_system(map_setup)
        .add_startup_system(setup)
        .add_system(camera_orbit)
        .run();
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SceneBundle {
        // scene: asset_server.load("levels/station.map"),
        scene: asset_server.load("levels/simple.map"),
        // scene: asset_server.load("levels/in_hull.map"),
        // scene: asset_server.load("levels/cube.map"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 24.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    // Scene
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("scenes/axis.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
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
