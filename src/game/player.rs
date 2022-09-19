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
    kinematic.collider = Collider::ball(0.3);
    kinematic.transform = TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0));

    commands
        .spawn()
        .insert_bundle(PlayerBundle {
            kinematic,
            ..default()
        })
        .insert(MovementProperties {
            speed: 4.0,
            acceleration: 4.0,
            friction: 4.0,
        })
        .insert(KinematicInput::default())
        .with_children(|build| {
            build.spawn().insert_bundle(Camera3dBundle {
                projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                    fov: f32::to_radians(80.0),
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.3),
                ..default()
            });
        });
}

pub fn player_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut KinematicInput, &Transform), With<PlayerInput>>,
) {
    let (mut kinematic_input, transform) = match query.get_single_mut() {
        Ok(single) => single,
        Err(_) => return,
    };

    let movement = Vec3 {
        x: input_to_axis(input.pressed(KeyCode::A), input.pressed(KeyCode::D)),
        y: input_to_axis(
            input.pressed(KeyCode::LShift),
            input.pressed(KeyCode::Space),
        ),
        z: input_to_axis(input.pressed(KeyCode::W), input.pressed(KeyCode::S)),
    };

    kinematic_input.movement = transform.right() * movement.x + transform.up() * movement.y
        - transform.forward() * movement.z;
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
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<PlayerInput>>,
) {
    for mut transform in query.iter_mut() {
        for event in mouse_motion_events.iter() {
            transform.rotate_local_x(event.delta.y * -0.005);
            transform.rotate_local_y(event.delta.x * -0.005);
        }
        let roll = input_to_axis(input.pressed(KeyCode::E), input.pressed(KeyCode::Q));
        transform.rotate_local_z(roll * PI * time.delta_seconds());
    }
}
