use std::collections::HashSet;

use self::{
    loader::QMapLoader,
    types::MAP_SCALE, component::{MapPointEntity, WorldData},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod build;
mod component;
mod loader;
mod types;

pub struct QMapPlugin;

impl Plugin for QMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset_loader::<QMapLoader>()
            .add_event::<MapBuild>()
            .register_type::<Hull>()
            .register_type::<WorldData>()
            .register_type::<MapPointEntity>()
            .insert_resource(CreatedWorld::default())
            .add_system(collision_spawner)
            .add_system(map_event_emitter);
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

pub enum MapBuild {
    PointEntity(MapPointEntity),
}

// fn create_lights(mut events: EventReader<MapBuild>, mut commands: Commands) {
//     events.iter().for_each(|event| match event {
//         MapBuild::PointEntity(entity) => match entity.name.as_str() {
//             "light_point" => {
//                 let position = convert_coords(entity.position);
//                 commands.spawn().insert_bundle(PointLightBundle {
//                     point_light: PointLight {
//                         intensity: entity
//                             .properties
//                             .get("intensity")
//                             .unwrap_or(&"800.0".to_string())
//                             .parse::<f32>()
//                             .unwrap_or_default(),
//                         range: entity
//                             .properties
//                             .get("range")
//                             .unwrap_or(&"15.0".to_string())
//                             .parse::<f32>()
//                             .unwrap_or_default(),
//                         color: Color::hsl(0.50, 0.15, 0.7),
//                         ..default()
//                     },
//                     transform: Transform::from_xyz(position.x, position.y, position.z),
//                     ..default()
//                 });
//             }
//             _ => (),
//         },
//     })
// }

pub fn convert_coords(map_point: Vec3) -> Vec3 {
    Vec3 {
        x: map_point.x,
        y: map_point.z,
        z: -map_point.y,
    } * MAP_SCALE
}

#[derive(Default)]
pub struct CreatedWorld {
    pub maps: HashSet<usize>
}

fn map_event_emitter(
    map_query: Query<(Entity, &WorldData)>,
    point_entity_query: Query<(Entity, &MapPointEntity)>,
    parent_query: Query<&Parent, Option<With<WorldData>>>,
    mut created_worlds: ResMut<CreatedWorld>,
    events: EventWriter<MapBuild>,
) {
    for (root, world_data) in map_query.iter() {
        if let None = created_worlds.maps.get(&world_data.id) {
            created_worlds.maps.insert(world_data.id);
            for (child, map_entity) in point_entity_query.iter() {
                // Check if the entity is a child of map root
            }
        }
    }
}
