use std::f32::consts::PI;

use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

use super::kinematic::*;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct PlayerInput;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    control: PlayerInput,
    #[bundle]
    kinematic: KinematicBundle,
}

pub fn player_setup(mut commands: Commands) {
    let mut kinematic = KinematicBundle::default();
    kinematic.collider = Collider::capsule_y(0.9, 0.3);
    kinematic.transform = TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0));

    commands
        .spawn()
        .insert_bundle(PlayerBundle {
            kinematic,
            ..default()
        })
        .insert(KinematicInput::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|build| {
            build
                .spawn()
                .insert_bundle(Camera3dBundle {
                    projection: bevy::render::camera::Projection::Perspective(
                        PerspectiveProjection {
                            fov: f32::to_radians(80.0),
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 0.7, 0.0),
                    ..default()
                })
                .insert(PlayerInput {});
        });
}

pub fn player_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut KinematicInput, With<PlayerInput>>,
    camera_query: Query<&mut Transform, (With<Camera3d>, With<PlayerInput>)>,
) {
    let mut kinematic_input = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    let camera_transform = match camera_query.get_single() {
        Ok(single) => single,
        Err(_) => return,
    };

    let movement = Vec3 {
        x: input_to_axis(input.pressed(KeyCode::A), input.pressed(KeyCode::D)),
        y: 0.0,
        z: input_to_axis(input.pressed(KeyCode::W), input.pressed(KeyCode::S)),
    } * 5.0;

    let forward = camera_transform
        .forward()
        .reject_from(Vec3::Y)
        .normalize_or_zero();

    let right = camera_transform
        .right()
        .reject_from(Vec3::Y)
        .normalize_or_zero();

    kinematic_input.movement = forward * -movement.z + right * movement.x;
}

fn input_to_axis(negative: bool, positive: bool) -> f32 {
    if negative == positive {
        return 0.0;
    }
    if negative {
        -1.0
    } else {
        1.0
    }
}

pub fn player_camera(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, (With<Camera3d>, With<PlayerInput>)>,
) {
    for event in mouse_motion_events.iter() {
        for mut transform in query.iter_mut() {
            transform.rotate(Quat::from_rotation_y(event.delta.x * -0.005));
            transform.rotate_local_x(event.delta.y * -0.005)
        }
    }
}
