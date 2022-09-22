use std::collections::HashMap;

pub use bevy::prelude::*;
use shalrath::repr::{Properties, TrianglePlane};

pub const INVERSE_SCALE_FACTOR: f32 = 16.0;
pub const MAP_SCALE: f32 = 1.0 / INVERSE_SCALE_FACTOR;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

#[derive(Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl From<TrianglePlane> for Plane {
    fn from(plane: TrianglePlane) -> Self {
        let v0 = Vec3 {
            x: plane.v0.x,
            y: plane.v0.y,
            z: plane.v0.z,
        };
        let v1 = Vec3 {
            x: plane.v1.x,
            y: plane.v1.y,
            z: plane.v1.z,
        };
        let v2 = Vec3 {
            x: plane.v2.x,
            y: plane.v2.y,
            z: plane.v2.z,
        };

        let normal = (v0 - v1).cross(v2 - v1).normalize();
        let projected = v0.project_onto_normalized(normal);
        let distance = projected.length() * normal.dot(projected).signum();

        Plane { normal, distance }
    }
}

#[derive(Clone)]
pub struct Face {
    pub plane: Plane,
    pub texture: String,
    pub vertices: Vec<Vertex>,
}

impl Face {
    pub fn as_tuples(&self) -> (Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f32; 3]>) {
        let mut tuples: (Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f32; 3]>) =
            (Vec::new(), Vec::new(), Vec::new());
        for vert in self.vertices.iter() {
            tuples.0.push(vec3_to_arr(vert.position));
            tuples.1.push(vec2_to_arr(vert.uv));
            tuples.2.push(vec3_to_arr(vert.normal));
        }
        tuples
    }

    pub fn offset_to_origin(&self, origin: Vec3) -> Face {
        let mut face = self.clone();
        for vert in face.vertices.iter_mut() {
            vert.position -= origin;
        }
        face
    }
}

fn vec2_to_arr(vec: Vec2) -> [f32; 2] {
    [vec.x, vec.y]
}

fn vec3_to_arr(vec: Vec3) -> [f32; 3] {
    [vec.x, vec.y, vec.z]
}

pub struct PointEntity {
    pub name: String,
    pub position: Vec3,
    pub properties: HashMap<String, String>,
}

impl PointEntity {
    pub fn from_properties(props: &Properties) -> Option<Self> {
        if props.len() == 0 {
            return None;
        }
        let mut properties: HashMap<String, String> = HashMap::new();
        for property in props.iter() {
            properties.insert(property.key.clone(), property.value.clone());
        }
        let name = match properties.get("classname") {
            Some(value) => value,
            None => "missing_entity",
        }
        .to_string();
        let position = parse_position(match properties.get("origin") {
            Some(value) => value,
            None => "0 0 0",
        });
        Some(PointEntity {
            name,
            position,
            properties,
        })
    }
}

pub fn parse_position(value: &str) -> Vec3 {
    let position: Vec<&str> = value.split_ascii_whitespace().collect();
    if position.len() == 3 {
        Vec3 {
            x: position[0].parse::<f32>().unwrap_or_default(),
            y: position[1].parse::<f32>().unwrap_or_default(),
            z: position[2].parse::<f32>().unwrap_or_default(),
        }
    } else {
        Vec3::ZERO
    }
}
