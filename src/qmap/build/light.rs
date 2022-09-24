use bevy::prelude::*;

use crate::qmap::component::MapPointEntity;

pub fn build_point_light(world: &mut World, entity: MapPointEntity) {
    world.spawn().insert_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: entity
                .properties
                .get("intensity")
                .unwrap_or(&"800.0".to_string())
                .parse::<f32>()
                .unwrap_or_default(),
            range: entity
                .properties
                .get("range")
                .unwrap_or(&"15.0".to_string())
                .parse::<f32>()
                .unwrap_or_default(),
            color: Color::hsl(0.50, 0.15, 0.7),
            ..default()
        },
        transform: entity.transform,
        ..default()
    });
}
