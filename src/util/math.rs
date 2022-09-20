use bevy::prelude::*;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    (value - a) / (b - a)
}

pub fn vec3_lerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: lerp(a.x, b.x, t),
        y: lerp(a.y, b.y, t),
        z: lerp(a.z, b.z, t),
    }
}
