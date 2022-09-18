use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{import::ImporterPlugins, qmap::QMapPlugin};

use self::{
    kinematic::{kinematic_collisions, kinematic_movement},
    player::*,
};

mod kinematic;
mod player;

pub fn init() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ImporterPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(QMapPlugin)
        .add_startup_system(map_setup)
        .add_startup_system(setup)
        .add_startup_system(player_setup)
        .add_system(mouse_capture)
        .add_system(player_system)
        .add_system(player_camera)
        .add_system(kinematic_movement)
        .add_system(kinematic_collisions)
        .run();
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("levels/station.map"),
        // scene: asset_server.load("levels/simple.map"),
        // scene: asset_server.load("levels/in_hull.map"),
        // scene: asset_server.load("levels/cube.map"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn mouse_capture(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if mouse.just_pressed(MouseButton::Left) {
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
    }
}
