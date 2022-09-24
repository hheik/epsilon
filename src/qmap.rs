use self::{
    loader::QMapLoader,
    types::MAP_SCALE, component::MapPointEntity,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod build;
pub mod component;
mod loader;
mod types;

pub struct QMapPlugin;

impl Plugin for QMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset_loader::<QMapLoader>()
            .register_type::<Hull>()
            .register_type::<MapPointEntity>()
            .add_system(collision_spawner);
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

pub fn convert_coords(map_point: Vec3) -> Vec3 {
    Vec3 {
        x: map_point.x,
        y: map_point.z,
        z: -map_point.y,
    } * MAP_SCALE
}
