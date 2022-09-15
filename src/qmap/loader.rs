use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    utils::BoxedFuture,
};
use shalrath::repr::{Brush, Map, Point, TrianglePlane};

const INVERSE_SCALE_FACTOR: f32 = 16.0;
const MAP_SCALE: f32 = 1.0 / INVERSE_SCALE_FACTOR;

struct Vertex {
    position: Vec3,
}

struct Plane {
    normal: Vec3,
    distance: f32,
}

impl From<TrianglePlane> for Plane {
    fn from(plane: TrianglePlane) -> Self {
        let v0 = point_to_vec(plane.v0);
        let v1 = point_to_vec(plane.v1);
        let v2 = point_to_vec(plane.v2);

        let normal = (v0 - v1).cross(v2 - v1).normalize();
        let projected = v0.project_onto_normalized(normal);
        let distance = projected.length() * normal.dot(projected).signum();

        Plane { normal, distance }
    }
}

#[derive(Default)]
pub struct QMapLoader;

impl AssetLoader for QMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move { Ok(load_qmap(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}

async fn load_qmap<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<(), bevy::asset::Error> {
    let qmap = String::from_utf8(bytes.to_vec())
        .expect("Failed to parse map as utf-8")
        .parse::<Map>()
        .expect("Failed to parse map");

    let mut world = World::default();

    for entity in qmap.0.iter() {
        for prop in entity.properties.iter() {
            println!("[property] {}: {}", prop.key, prop.value);
        }
        for brush in entity.brushes.iter() {
            let vertices = vertices_from_brush(brush);

            for vertex in vertices {
                let mesh = Mesh::from(shape::Cube { size: 0.25 });
                let mesh = load_context.set_labeled_asset(&"Mesh0", LoadedAsset::new(mesh));

                let material = StandardMaterial { ..default() };
                let material =
                    load_context.set_labeled_asset("Material0", LoadedAsset::new(material));

                world.spawn().insert_bundle(PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_xyz(
                        vertex.position.x * MAP_SCALE,
                        vertex.position.z * MAP_SCALE,
                        -vertex.position.y * MAP_SCALE,
                    ),
                    ..default()
                });
            }
        }
    }

    let scene = Scene::new(world);

    load_context.set_default_asset(LoadedAsset::new(scene));

    Ok(())
}

fn vertices_from_brush(brush: &Brush) -> Vec<Vertex> {
    let mut vertices: Vec<Vertex> = vec![];
    let planes: Vec<_> = brush
        .0
        .iter()
        .clone()
        .map(|brush_plane| Plane::from(brush_plane.plane))
        .collect();
    for p1 in brush.0.iter() {
        for p2 in brush.0.iter() {
            for p3 in brush.0.iter() {
                if let Some(point) = plane_intersection(
                    Plane::from(p1.plane),
                    Plane::from(p2.plane),
                    Plane::from(p3.plane),
                ) {
                    let point = point;
                    if vertex_in_hull(point, &planes)
                        && !vertices.iter().any(|v| v.position.abs_diff_eq(point, 0.01))
                    {
                        vertices.push(Vertex { position: point })
                    }
                };
            }
        }
    }
    vertices
}

fn point_to_vec(point: Point) -> Vec3 {
    Vec3 {
        x: point.x,
        y: point.y,
        z: point.z,
    }
}

/// This mystery algorithm was provided by
/// https://gdbooks.gitbooks.io/3dcollisions/content/Chapter1/three_plane_intersection.html
fn plane_intersection(p1: Plane, p2: Plane, p3: Plane) -> Option<Vec3> {
    let m1 = Vec3 {
        x: p1.normal.x,
        y: p2.normal.x,
        z: p3.normal.x,
    };
    let m2 = Vec3 {
        x: p1.normal.y,
        y: p2.normal.y,
        z: p3.normal.y,
    };
    let m3 = Vec3 {
        x: p1.normal.z,
        y: p2.normal.z,
        z: p3.normal.z,
    };
    let d = Vec3 {
        x: p1.distance,
        y: p2.distance,
        z: p3.distance,
    };

    let u = m2.cross(m3);
    let v = m1.cross(d);

    let denom = m1.dot(u);

    if denom.abs() < f32::EPSILON {
        return None;
    }

    Some(
        Vec3 {
            x: d.dot(u),
            y: m3.dot(v),
            z: -m2.dot(v),
        } / denom,
    )
}

fn vertex_in_hull(point: Vec3, faces: &Vec<Plane>) -> bool {
    !faces.iter().any(|face| {
        let projection = face.normal.dot(point);
        (projection - face.distance) > 0.01
    })
}

fn get_vertex_uv(point: Vec3, face: Plane, angle: f32, scale: Vec2) -> Vec2 {
    let dot_x = face.normal.dot(Vec3::X).abs();
    let dot_y = face.normal.dot(Vec3::Y).abs();
    let dot_z = face.normal.dot(Vec3::Z).abs();

    let mut uv = if dot_x >= dot_y && dot_x >= dot_z {
        Vec2 {
            x: point.y,
            y: -point.z,
        }
    } else if dot_y >= dot_x && dot_y >= dot_z {
        Vec2 {
            x: point.x,
            y: -point.z,
        }
    } else {
        Vec2 {
            x: point.x,
            y: -point.y,
        }
    };

    uv = Vec2 {
        x: uv.x * angle.cos() - uv.y * angle.sin(),
        y: uv.x * angle.sin() - uv.y * angle.cos(),
    };

    uv
}
