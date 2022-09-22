use std::f32::consts::PI;

use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::qmap::{convert_coords, MapBuild};

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
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut KinematicInput, &Transform), With<PlayerInput>>,
) {
    for (mut kinematic, transform) in query.iter_mut() {
        let mut x = 0.0;
        let mut y = 0.0;
        let z = input_to_axis(input.pressed(KeyCode::E), input.pressed(KeyCode::Q)) * PI * 0.5;
        for event in mouse_motion_events.iter() {
            x += event.delta.y * -0.1;
            y += event.delta.x * -0.1;
        }
        kinematic.turning = transform.rotation * Vec3 { x, y, z };
    }
}

pub fn player_spawn(mut events: EventReader<MapBuild>, mut commands: Commands) {
    events.iter().for_each(|event| match event {
        MapBuild::Entity(entity) => match entity.name.as_str() {
            "info_player_start" => {
                println!("event");
                let position = convert_coords(entity.position);

                let radius = 0.3;
                let mut kinematic = KinematicBundle::default();
                kinematic.collider = Collider::ball(radius);
                kinematic.transform =
                    TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z));

                commands
                    .spawn()
                    .insert_bundle(PlayerBundle {
                        kinematic,
                        ..default()
                    })
                    .insert(KinematicProperties {
                        speed: 4.0,
                        acceleration: 4.0,
                        friction: 4.0,
                        turning_lerp: 20.0,
                    })
                    .insert(KinematicInput::default())
                    .with_children(|build| {
                        build.spawn().insert_bundle(Camera3dBundle {
                            projection: bevy::render::camera::Projection::Perspective(
                                PerspectiveProjection {
                                    fov: f32::to_radians(80.0),
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });
            }
            _ => println!("event???"),
        },
    })
}
