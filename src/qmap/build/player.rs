use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::qmap::component::MapPointEntity;
use crate::game::kinematic::*;
use crate::game::player::*;

pub fn build_player(world: &mut World, entity: MapPointEntity) {
    let radius = 0.3;
    let mut kinematic = KinematicBundle::default();
    kinematic.collider = Collider::ball(radius);
    kinematic.transform = TransformBundle {
        local: entity.transform,
        ..default()
    };

    world
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
