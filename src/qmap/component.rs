use std::collections::HashMap;

use bevy::prelude::*;
use shalrath::repr::Properties;

#[derive(Default, Component, Clone, Reflect)]
#[reflect(Component)]
pub struct MapPointEntity {
    pub name: String,
    pub transform: Transform,
    #[reflect(ignore)]
    pub properties: HashMap<String, String>,
}

impl MapPointEntity {
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
        let translation = parse_position(match properties.get("origin") {
            Some(value) => value,
            None => "0 0 0",
        });
        let rotation = parse_angle(match properties.get("angle") {
            Some(value) => value,
            None => "0",
        });
        let transform = Transform {
            translation,
            rotation,
            ..default()
        };
        Some(MapPointEntity {
            name,
            transform,
            properties,
        })
    }
}

pub fn parse_position(value: &str) -> Vec3 {
    let position: Vec<&str> = value.split_ascii_whitespace().collect();
    if position.len() == 3 {
        Vec3 {
            x: position[0].parse::<f32>().expect("Invalid position value"),
            y: position[1].parse::<f32>().expect("Invalid position value"),
            z: position[2].parse::<f32>().expect("Invalid position value"),
        }
    } else {
        Vec3::ZERO
    }
}

pub fn parse_angle(value: &str) -> Quat {
    let angle = value.parse::<f32>().expect("Invalid angle value") - 90.0;
    let angle = angle.to_radians();
    // TODO: check if this and/or player camera is wrong
    Quat::from_rotation_y(angle)
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
pub struct WorldData {
    pub id: usize,
}
