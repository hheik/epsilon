use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::util::math::{inverse_lerp, lerp};

#[derive(Bundle)]
pub struct KinematicBundle {
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub events: ActiveEvents,
    pub collisions: ActiveCollisionTypes,
    #[bundle]
    pub transform: TransformBundle,
}

impl Default for KinematicBundle {
    fn default() -> Self {
        KinematicBundle {
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            gravity_scale: GravityScale(0.0),
            collider: Collider::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            events: ActiveEvents::COLLISION_EVENTS,
            collisions: ActiveCollisionTypes::all(),
            transform: TransformBundle::default(),
        }
    }
}

#[derive(Default, Component)]
pub struct MovementProperties {
    pub speed: f32,
    pub acceleration: f32,
    pub friction: f32,
}

#[derive(Default, Component)]
pub struct KinematicInput {
    pub movement: Vec3,
    pub facing: Quat,
}

pub fn kinematic_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut Velocity,
        Option<&KinematicInput>,
        Option<&MovementProperties>,
    )>,
) {
    for (mut velocity, input, props) in query.iter_mut() {
        let default = &KinematicInput::default();
        let input = input.unwrap_or(default);

        let default = &MovementProperties {
            speed: 1.0,
            acceleration: 20.0,
            friction: 40.0,
        };
        let props = props.unwrap_or(default);

        let current_velocity = velocity.linvel;
        let target_velocity = input.movement * props.speed;

        let angle_lerp = if current_velocity.length_squared() > 0.01 {
            let result = inverse_lerp(
                0.0,
                PI,
                current_velocity
                    .angle_between(target_velocity - current_velocity)
                    .abs(),
            );
            if result.is_nan() {
                0.0
            } else {
                result
            }
        } else {
            0.0
        };
        let delta_interpolation = angle_lerp.clamp(0.0, 1.0);
        let velocity_change_speed =
            lerp(props.acceleration, props.friction, delta_interpolation) * props.speed;

        velocity.linvel = move_towards(
            current_velocity,
            target_velocity,
            velocity_change_speed * time.delta_seconds(),
        );
    }
}

pub fn kinematic_collisions(mut collision_events: EventReader<CollisionEvent>) {
    // TODO: Possibly use KinematicVelocityBased rigidbody and handle collisions?
    // for event in collision_events.iter() {
    //     println!("collision: {event:?}");
    // }
}

fn move_towards(from: Vec3, to: Vec3, amount: f32) -> Vec3 {
    let diff = to - from;
    let length = diff.length();
    if length <= f32::EPSILON {
        return from;
    }
    from + diff.normalize() * length.min(amount)
}
