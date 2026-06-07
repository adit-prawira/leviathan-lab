use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::editor::resource::TransformContext;
use crate::history::edit_history::{Action, EditHistory, EditHistoryManager};

use super::body_part::BodyPart;

#[derive(Resource, Default)]
pub struct BodyHierarchy {
    pub entities: HashMap<u32, Entity>,
}

pub struct HierarchyReference<'a>{
    pub children_map: &'a HashMap<Entity, Vec<Entity>>,
    pub body_parts: &'a Query<'a, 'a, (Entity, &'static BodyPart)>,
    pub children_of: &'a Query<'a, 'a, &'static ChildOf>,
}

pub struct EntityReference<'a> {
    pub selected_entity: &'a Entity,
    pub entity: &'a Entity
}

impl BodyHierarchy {
    pub fn unique_name(base: &str, existing_names: &HashSet<&str>) -> String {
        if !existing_names.contains(&base) {
            return base.to_string();
        }
        let mut i = 1u32;
        loop {
            let name = format!("{}.{:03}", base, i);
            if !existing_names.contains(&name.as_str()) {
                return name;
            }
            i += 1;
        }
    }

    pub fn is_descendant(candidate: Entity, of: Entity, children_of: &Query<&ChildOf>) -> bool {
        let mut current = candidate;
        loop {
            match children_of.get(current) {
                Ok(child_of) if child_of.parent() == of => return true,
                Ok(child_of) => current = child_of.parent(),
                Err(_) => return false
            }
        };
    }

    pub fn assign_to_root(
        dragged_entity: &Entity,
        hierarchy_reference: &HierarchyReference,
        transform_ctx: &mut TransformContext, 
        commands: &mut Commands,
        history: &mut EditHistory
    ) {
        let dragged = *dragged_entity;
        let Ok((_, dragged_body_part)) = hierarchy_reference.body_parts.get(dragged) else {return;};

        if let Some(old_parent) = hierarchy_reference.children_of.get(dragged).ok().map(|children| children.parent()) {
            let old_parent_id = hierarchy_reference.body_parts.get(old_parent).ok().map(|(_, body_part)| body_part.id); 
            let old_transform = transform_ctx.transform_query.get(dragged).copied().unwrap_or_default();
            let old_world = transform_ctx.global_transform_query.get(dragged).copied().unwrap_or_default();
            let old_world_matrix = Mat4::from(old_world.affine());
            let new_transform = Transform::from_matrix(old_world_matrix);
            if let Ok(mut transform) = transform_ctx.transform_query.get_mut(dragged) {
                *transform = new_transform;
            }
            commands.entity(dragged).remove::<ChildOf>();
            
            EditHistoryManager::record(history, Action::AssignParentEntity { 
                entity: dragged,
                entity_id: dragged_body_part.id,
                old_parent: Some(old_parent),
                old_parent_id, 
                old_transform, 
                new_parent: None, 
                new_transform,
                new_parent_id: None,
            }); 
        } 
    } 

    pub fn assign_to_parent(
        dragged_entity: &Entity,
        entity_reference: &EntityReference, 
        hierarchy_reference: &HierarchyReference,
        transform_ctx: &mut TransformContext,
        commands: &mut Commands,
        history: &mut EditHistory         
    ) {
        let entity = entity_reference.entity; 
        let dragged = *dragged_entity;
        let should_change_hierarychy = dragged != *entity
            && !Self::is_descendant(*entity, dragged, hierarchy_reference.children_of);
        let Ok((_, dragged_body_part)) = hierarchy_reference.body_parts.get(dragged) else {return;};
        let Ok((_, target_body_part)) = hierarchy_reference.body_parts.get(*entity) else {return;};

        if should_change_hierarychy { 
            let old_parent = hierarchy_reference.children_of.get(dragged).ok().map(|child_of| child_of.parent());
            let old_parent_id = old_parent
                .and_then(|parent| hierarchy_reference.body_parts.get(parent).ok())
                .map(|(_, body_part)| body_part.id);
            let old_world = transform_ctx.global_transform_query.get(dragged).copied().unwrap_or_default();
            let old_transform = transform_ctx.transform_query.get(dragged).copied().unwrap_or_default();
            
            let parent_world = transform_ctx.global_transform_query.get(*entity).copied().unwrap_or_default();
            let local_parent_matrix = Mat4::from(parent_world.affine().inverse() * old_world.affine());
            let new_parent = Some(*entity);
            let new_transform = Transform::from_matrix(local_parent_matrix);

            if let Ok(mut transform) = transform_ctx.transform_query.get_mut(dragged) {
                *transform = new_transform;
            }

            commands.entity(*entity).add_child(dragged);
            
            EditHistoryManager::record(history, Action::AssignParentEntity { 
                entity: dragged,
                entity_id: dragged_body_part.id,
                old_parent,
                old_parent_id,
                old_transform, 
                new_parent, 
                new_transform,
                new_parent_id: Some(target_body_part.id),
            }); 
        };
    }
}
