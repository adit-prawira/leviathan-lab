use bevy::prelude::*;
use crate::model::body_hierarchy::{BodyHierarchy};
use crate::model::body_part::{PartType};
use crate::model::monster::Monster;
use crate::selector::{OriginalMaterial, Selector};
use std::collections::HashMap;

pub struct Spawner;

impl Spawner {
    pub fn spawn_monster(
        mut commands:Commands, 
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>
    ) {
        let monster: Monster = Monster::default_hatchling();
        let bevy_materials: Vec<Handle<StandardMaterial>> = monster.materials.iter()
            .map(|material| materials.add(StandardMaterial{
                base_color: Color::srgba(
                    material.base_color[0], material.base_color[1], 
                    material.base_color[2], material.base_color[3]
                ),
                perceptual_roughness: material.roughness,
                metallic: material.metallic,
                ..default()
            })).collect();

        let mut entity_map: HashMap<u32, Entity> = HashMap::new();
        for part in &monster.parts {
            let mesh: Mesh = match &part.part_type {
                PartType::Sphere { radius } => Sphere::new(*radius).mesh().build(),
                PartType::Capsule { radius, half_length } => Capsule3d{
                    radius: *radius, 
                    half_length: *half_length
                }.mesh().build()
            };

            let material_handler:Handle<StandardMaterial> = bevy_materials[part.material_id as usize].clone();
            let entity: Entity = commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material_handler.clone()),
                Transform::from_translation(Vec3::from_array(part.translation))
                  .with_rotation(Quat::from_array(part.rotation))
                  .with_scale(Vec3::from_array(part.scale)),
                part.clone(),
                OriginalMaterial(material_handler)
            ))
            .observe(Selector::on_press) // detect on pointer clicked on entity
            .id();
            entity_map.insert(part.id, entity);
        }

        let mut hierarchy: BodyHierarchy = BodyHierarchy::default();
        for (id, entity) in &entity_map {
            hierarchy.entities.insert(*id, *entity);
        }
        commands.insert_resource(hierarchy);

        // Sync monster part children-parent relationship to Bevy's ECS hierarchy
        for part in &monster.parts {
            let Some(parent_id) = part.parent_id else {continue};
            let Some(&child_entity) = entity_map.get(&part.id) else {continue};
            let Some(&parent_entity) = entity_map.get(&parent_id) else {continue};
            commands.entity(parent_entity).add_child(child_entity);
        }
    }
}
