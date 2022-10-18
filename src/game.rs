use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

use crate::{import::ImporterPlugins, qmap::QMapPlugin};

use self::{kinematic::kinematic_movement, light::*, player::*};

pub mod hierarchy;
pub mod kinematic;
pub mod light;
pub mod player;

pub fn init() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ImporterPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(QMapPlugin)
        .add_plugin(LightPlugin)
        // .add_plugin(HierarchyVisualizerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(map_setup)
        .add_startup_system(setup)
        .add_system(mouse_capture)
        .add_system(player_spawn)
        .add_system(player_system)
        .add_system(player_camera)
        .add_system(kinematic_movement)
        .run();
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("levels/station.map"),
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
    commands
        .spawn()
        .insert(Name::new("axis mesh"))
        .insert_bundle(SceneBundle {
            scene: asset_server.load("scenes/axis.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
}

fn mouse_capture(
    mut windows: ResMut<Windows>,
    keycode: Res<Input<KeyCode>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if keycode.just_pressed(KeyCode::P) {
        window.set_cursor_visibility(!window.cursor_visible());
        window.set_cursor_lock_mode(!window.cursor_locked());
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
    }
}
