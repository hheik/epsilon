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

pub fn convert_coords(map_point: Vec3) -> Vec3 {
    Vec3 {
        x: map_point.x,
        y: map_point.z,
        z: -map_point.y,
    } * MAP_SCALE
}

// TODO: change to use Added<T> change detection
#[derive(Default)]
pub struct CreatedWorld {
    pub maps: HashSet<usize>
}

fn map_event_emitter(
    map_query: Query<&WorldData>,
    point_entity_query: Query<(&Parent, &MapPointEntity)>,
    parent_query: Query<(Option<&Parent>, Option<&WorldData>)>,
    mut created_worlds: ResMut<CreatedWorld>,
    mut events: EventWriter<MapBuild>,
) {
    for root_world_data in map_query.iter() {
        if let None = created_worlds.maps.get(&root_world_data.id) {
            created_worlds.maps.insert(root_world_data.id);
            for (entity, map_entity) in point_entity_query.iter() {
                // Check if the entity is a child of map root
                let mut found = false;
                let mut current = entity;
                loop {
                    if let Ok((parent, world_data)) = parent_query.get(current.get()) {
                        if let Some(world_data) = world_data {
                            if world_data.id == root_world_data.id {
                                found = true;
                                break;
                            }
                            if let Some(parent) = parent {
                                current = parent;
                            } else {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                };
                if found {
                    events.send(MapBuild::PointEntity(map_entity.clone()));
                }
            }
        }
    }
}
