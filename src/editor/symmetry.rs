use crate::model::body_material::BodyMaterial;
use bevy::prelude::*;

const MIRROR_THRESHOLD: f32 = 0.05;

#[derive(Resource, Default)]
pub struct SymmetryMode {
    pub enabled: bool,
}

type MaterialSnapshot = (Entity, BodyMaterial, Vec3);

#[derive(Resource, Default)]
pub struct PendingSymmetricChanges {
    pub material_snapshots: Vec<MaterialSnapshot>,
}
pub struct Symmetry;

/*
* Symmetry mode resource and system.
+ When enabled, material changes (color, roughness, metallic)
+ on a selected part are automatically applied to its mirror
+ counterpart on the opposite X side.
+ */
impl Symmetry {
    pub fn collect_changes(
        changes: Query<(Entity, &BodyMaterial, &GlobalTransform), Changed<BodyMaterial>>,
        mut pending_symmetric_changes: ResMut<PendingSymmetricChanges>,
    ) {
        pending_symmetric_changes.material_snapshots = changes
            .iter()
            .map(|(entity, body_material, global_transform)| {
                (
                    entity,
                    body_material.clone(),
                    global_transform.translation(),
                )
            })
            .collect();
    }

    pub fn apply(
        mode: Res<SymmetryMode>,
        pending_symmetric_changes: Res<PendingSymmetricChanges>,
        all_body_materials: Query<(Entity, &GlobalTransform)>,
        mut body_materials: Query<&mut BodyMaterial>,
    ) {
        if !mode.enabled { return; }

        for (entity, body_material, current_entity_position) in
            &pending_symmetric_changes.material_snapshots
        {
            if current_entity_position.x.abs() < MIRROR_THRESHOLD { continue; }

            let mirrored_entity = all_body_materials
                .iter()
                .filter(|(e, _)| *e != *entity) // exclude the current entity
                .find(|(_, other_entity_global_transform)| {
                    let other_entity_position = other_entity_global_transform.translation();
                    (other_entity_position.x + current_entity_position.x).abs() < MIRROR_THRESHOLD
                        && (other_entity_position.y - current_entity_position.y).abs()
                            < MIRROR_THRESHOLD
                        && (other_entity_position.z - current_entity_position.z).abs()
                            < MIRROR_THRESHOLD
                })
                .map(|(e, _)| e);

            let Some(mirrored_entity) = mirrored_entity else { continue; };
            let Ok(mut mirrored_material) = body_materials.get_mut(mirrored_entity) else { continue; };
            *mirrored_material = body_material.clone();
        }
    }
}
