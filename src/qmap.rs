use self::{
    loader::QMapLoader,
    types::{PointEntity, MAP_SCALE},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod entity;
mod loader;
mod types;

pub struct QMapPlugin;

impl Plugin for QMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset_loader::<QMapLoader>()
            .register_type::<Hull>()
            .add_event::<MapBuild>()
            .add_system(collision_spawner)
            .add_system(map_events)
            .add_system(create_lights);
    }
}

/// Rapier colliders don't implement reflections (required by scene builder),
/// so we store the hull data in a component and then have a system add the colliders
#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Hull {
    pub points: Vec<Vec3>,
}

fn collision_spawner(
    mut query: Query<Entity, (With<Hull>, Without<Collider>)>,
    hull_query: Query<&Hull>,
    mut commands: Commands,
) {
    for entity in query.iter_mut() {
        let hull = hull_query.get(entity).unwrap();
        let collider =
            Collider::convex_hull(&hull.points[..]).expect("Failed to create collider for brush");
        commands.entity(entity).insert(collider);
    }
}

fn map_events() {
    let mut events = Events::<MapBuild>::default();
    events.update();
}

pub enum MapBuild {
    Entity(PointEntity),
}

fn create_lights(mut events: EventReader<MapBuild>, mut commands: Commands) {
    events.iter().for_each(|event| match event {
        MapBuild::Entity(entity) => match entity.name.as_str() {
            "light_point" => {
                let position = convert_coords(entity.position);
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
                    transform: Transform::from_xyz(position.x, position.y, position.z),
                    ..default()
                });
            }
            _ => (),
        },
    })
}

pub fn convert_coords(map_point: Vec3) -> Vec3 {
    Vec3 {
        x: map_point.x,
        y: map_point.z,
        z: -map_point.y,
    } * MAP_SCALE
}
