use std::f32::consts::PI;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    utils::BoxedFuture,
};
use shalrath::repr::{Brush, Map, TextureOffset};

use super::{types::*, entity::*};

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
            let faces = faces_from_brush(brush).iter().map(convert_face_coords).collect();
            build_brush_entity(&mut world, load_context, faces);
        }
    }

    let scene = Scene::new(world);

    load_context.set_default_asset(LoadedAsset::new(scene));

    Ok(())
}

fn faces_from_brush(brush: &Brush) -> Vec<Face> {
    let mut faces: Vec<Face> = vec![];
    let planes: Vec<_> = brush
        .0
        .iter()
        .clone()
        .map(|brush_plane| Plane::from(brush_plane.plane))
        .collect();
    for p1 in brush.0.iter() {
        let mut vertices: Vec<Vertex> = vec![];
        let plane = Plane::from(p1.plane);
        for p2 in brush.0.iter() {
            for p3 in brush.0.iter() {
                if let Some(position) =
                    plane_intersection(plane, Plane::from(p2.plane), Plane::from(p3.plane))
                {
                    if vertex_in_hull(position, &planes)
                        && !vertices
                            .iter()
                            .any(|v| v.position.abs_diff_eq(position, 0.01))
                    {
                        let uv = get_vertex_uv(
                            position,
                            plane,
                            match p1.texture_offset {
                                TextureOffset::Standard { u, v } => Vec2 { x: u, y: v },
                                TextureOffset::Valve { u: _u, v: _v } => Vec2::ZERO,
                            },
                            p1.angle,
                            Vec2 {
                                x: p1.scale_x,
                                y: p1.scale_y,
                            },
                        );
                        vertices.push(Vertex {
                            position,
                            normal: plane.normal,
                            uv,
                        })
                    }
                };
            }
        }
        order_vertices_counter_clockwise(plane.normal, &mut vertices);
        faces.push(Face {
            plane,
            vertices,
            texture: p1.texture.clone(),
        });
    }
    faces
}

fn convert_face_coords(face: &Face) -> Face {
    Face {
        plane: Plane {
            distance: face.plane.distance,
            normal: convert_coords(face.plane.normal).normalize(),
        },
        texture: face.texture.clone(),
        vertices: face.vertices.iter().map(|vert| Vertex { 
            position: convert_coords(vert.position),
            normal: convert_coords(vert.normal).normalize(),
            uv: vert.uv,
        }).collect(),
    }
}

fn convert_coords(map_point: Vec3) -> Vec3 {
    Vec3 {
        x: map_point.x,
        y: map_point.z,
        z: -map_point.y,
    } * MAP_SCALE
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

fn get_vertex_uv(point: Vec3, face: Plane, offset: Vec2, angle: f32, scale: Vec2) -> Vec2 {
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

    // TODO: calculate actual texture size
    let texture_size = Vec2 { x: 64.0, y: 64.0 };
    uv /= texture_size;
    uv /= scale;
    uv += offset / texture_size;

    uv
}

fn order_vertices_counter_clockwise(normal: Vec3, vertices: &mut Vec<Vertex>) {
    let mut min: Option<Vec3> = None;
    let mut max: Option<Vec3> = None;
    for vertex in vertices.iter() {
        min = match min {
            Some(min) => Some(min.min(vertex.position)),
            None => Some(vertex.position),
        };
        max = match max {
            Some(max) => Some(max.max(vertex.position)),
            None => Some(vertex.position),
        };
    }

    let center = match (min, max) {
        (Some(min), Some(max)) => (min + max) / 2.0,
        (_, _) => return,
    };

    let axis = (vertices.first().unwrap().position - center).normalize();

    vertices.sort_unstable_by(|a, b| {
        let a = angle_around_axis(normal, axis, a.position - center);
        let b = angle_around_axis(normal, axis, b.position - center);
        a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
    });
}

fn normalize_if_not(vector: Vec3) -> Vec3 {
    match vector.is_normalized() {
        true => vector,
        false => vector.normalize(),
    }
}

fn angle_around_axis(axis: Vec3, from: Vec3, to: Vec3) -> f32 {
    angle_around_axis_normalized(
        normalize_if_not(axis),
        normalize_if_not(from),
        normalize_if_not(to)
    )
}

/// Get the clockwise angle between 2 vectors in [0, 360[ range
fn angle_around_axis_normalized(normal: Vec3, from: Vec3, to: Vec3) -> f32 {
    let angle = from.angle_between(to);
    if normal.dot(from.cross(to)) >= 0.0 {
        angle
    } else {
        (PI * 2.0 - angle) % (PI * 2.0)
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::qmap::loader::angle_around_axis;

    #[test]
    fn greater_than_180_degrees() {
        assert_eq!(Vec3::Z, Vec3::X.cross(Vec3::Y));

        let a = Vec3::X;
        let b = Vec3::Y;
        let normal = Vec3::Z;

        assert_eq!(0.0, angle_around_axis(normal, a, a).to_degrees().round());
        assert_eq!(45.0, angle_around_axis(normal, a, a+b).to_degrees().round());
        assert_eq!(90.0, angle_around_axis(normal, a, b).to_degrees().round());
        assert_eq!(135.0, angle_around_axis(normal, a, b-a).to_degrees().round());
        assert_eq!(180.0, angle_around_axis(normal, a, -a).to_degrees().round());
        assert_eq!(225.0, angle_around_axis(normal, a, -a-b).to_degrees().round());
        assert_eq!(270.0, angle_around_axis(normal, a, -b).to_degrees().round());
        assert_eq!(315.0, angle_around_axis(normal, a, a-b).to_degrees().round());
    }

    #[test]
    fn inverted_normal() {
        assert_eq!(Vec3::Z, Vec3::X.cross(Vec3::Y));

        let a = Vec3::X;
        let b = Vec3::Y;
        let normal = Vec3::NEG_Z;

        assert_eq!(0.0, angle_around_axis(normal, a, a).to_degrees().round());
        assert_eq!(360.0 - 45.0, angle_around_axis(normal, a, a+b).to_degrees().round());
        assert_eq!(360.0 - 90.0, angle_around_axis(normal, a, b).to_degrees().round());
        assert_eq!(360.0 - 135.0, angle_around_axis(normal, a, b-a).to_degrees().round());
        assert_eq!(360.0 - 180.0, angle_around_axis(normal, a, -a).to_degrees().round());
        assert_eq!(360.0 - 225.0, angle_around_axis(normal, a, -a-b).to_degrees().round());
        assert_eq!(360.0 - 270.0, angle_around_axis(normal, a, -b).to_degrees().round());
        assert_eq!(360.0 - 315.0, angle_around_axis(normal, a, a-b).to_degrees().round());
    }

    #[test]
    fn different_comparison() {
        assert_eq!(Vec3::Z, Vec3::X.cross(Vec3::Y));

        let a = Vec3::NEG_X + Vec3::NEG_Y;
        let b = Vec3::Y;
        let normal = Vec3::Z;

        assert_eq!(0.0, angle_around_axis(normal, a, a).to_degrees().round());
        assert_eq!(225.0, angle_around_axis(normal, a, b).to_degrees().round());
        assert_eq!(45.0, angle_around_axis(normal, a, -b).to_degrees().round());
    }
}
