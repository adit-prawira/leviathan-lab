use bevy::mesh::Indices;
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;
use parry3d::math::Vector;
use parry3d::query::Ray;
use parry3d::query::RayCast;
use std::collections::HashMap;

use parry3d::shape::TriMesh;

#[derive(Resource, Default)]
pub struct BvhCache {
    pub meshes: HashMap<Entity, TriMesh>
}

pub struct BvhManager;

impl BvhManager {
    pub fn handle_rebuild(
        mesh_assets: Res<Assets<Mesh>>, 
        mesh_query: Query<(Entity, &Mesh3d), Added<Mesh3d>>,
        mut cache: ResMut<BvhCache>
    ) {  
        for (entity, mesh3d) in &mesh_query {
            let handle = &mesh3d.0;
            let Some(mesh) = mesh_assets.get(handle) else {continue;};
            let Some(trimesh) = Self::build_trimesh(mesh) else {continue;};
            cache.meshes.insert(entity, trimesh);
        }
    }
    
    pub fn rebuild_for_entity(mesh: &Mesh, entity: Entity, cache: &mut BvhCache) {
        let Some(trimesh) = Self::build_trimesh(mesh) else {return;};
        cache.meshes.insert(entity, trimesh);
    }

    pub fn intersect_mesh(cache: &BvhCache, ray: &Ray3d, entity: Entity, entity_world: &GlobalTransform) -> Option<(Vec3, Vec3)> {
        let trimesh = cache.meshes.get(&entity)?;
        let affine = entity_world.affine();
        let inverse = affine.inverse();
        let local_origin = inverse.transform_point3(ray.origin);
        let local_direction = inverse.transform_vector3(*ray.direction);
        let local_ray = Ray::new(
            Vector::new(local_origin[0], local_origin[1], local_origin[2]),
            Vector::new(local_direction[0], local_direction[1], local_direction[2])
        );
        let intersection = trimesh.cast_local_ray_and_get_normal(&local_ray, f32::MAX, true)?;
        let local_point = local_ray.point_at(intersection.time_of_impact);
        let point = affine.transform_point3(Vec3::new(local_point.x, local_point.y, local_point.z));
        
        let local_normal = Vec3::new(intersection.normal.x, intersection.normal.y, intersection.normal.z);
        let normal = Vec3::from(affine.matrix3.inverse().transpose() * Vec3A::from(local_normal)).normalize();

        Some((point, normal))
    }

    fn build_trimesh(mesh: &Mesh) -> Option<TriMesh> {
        let VertexAttributeValues::Float32x3(vertices) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)? else {return None;};
        let indices = mesh.indices()?;
        let position_vectors: Vec<Vector> = vertices.iter()
            .map(|vertex| Vector::new(vertex[0], vertex[1], vertex[2]))
            .collect();
        let triangles: Vec<[u32; 3]> = match indices {
            Indices::U32(index) => index.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect(),
            Indices::U16(index) => index.chunks_exact(3).map(|c| [c[0] as u32, c[1] as u32, c[2] as u32]).collect()
        };
        TriMesh::new(position_vectors, triangles).ok() 
    }
}
