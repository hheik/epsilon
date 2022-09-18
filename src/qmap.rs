use self::loader::QMapLoader;
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
            .add_system(collision_spawner);
    }
}

/// Rapier colliders don't implement reflections (required by scene builder),
/// so we store the hull data in a component and then have a system add the colliders
#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Hull {
    points: Vec<Vec3>,
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
