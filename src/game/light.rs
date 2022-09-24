use bevy::prelude::*;

use crate::qmap::MapBuild;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(light_builder);
    }
}

fn light_builder(mut events: EventReader<MapBuild>, mut commands: Commands) {
    events.iter().for_each(|event| match event {
        MapBuild::PointEntity(entity) => match entity.name.as_str() {
            "light_point" => {
                commands.spawn().insert_bundle(PointLightBundle {
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
                    transform: entity.transform.clone(),
                    ..default()
                });
            }
            _ => (),
        },
    })
}
