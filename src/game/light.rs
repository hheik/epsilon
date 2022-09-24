use bevy::prelude::*;

use crate::qmap::component::MapPointEntity;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(light_builder);
    }
}

fn light_builder(mut commands: Commands, query: Query<(Entity, &MapPointEntity), Added<MapPointEntity>>) {
    for (entity, map_point_entity) in query.iter() {
        match map_point_entity.name.as_str() {
            "light_point" => {
                commands.entity(entity).with_children(|builder| {
                    builder.spawn()
                        .insert(Name::new("point light"))
                        .insert_bundle(PointLightBundle {
                            point_light: PointLight {
                                intensity: map_point_entity
                                    .properties
                                    .get("intensity")
                                    .unwrap_or(&"800.0".to_string())
                                    .parse::<f32>()
                                    .unwrap_or_default(),
                                range: map_point_entity
                                    .properties
                                    .get("range")
                                    .unwrap_or(&"15.0".to_string())
                                    .parse::<f32>()
                                    .unwrap_or_default(),
                                color: Color::hsl(0.50, 0.15, 0.7),
                                ..default()
                            },
                            ..default()
                        });
                });
            },
            _ => ()
        }
    }
}
