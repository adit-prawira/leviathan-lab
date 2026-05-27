use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::editor::selector::{OriginalMaterial, Selector};
use crate::model::body_hierarchy::BodyHierarchy;
use crate::model::body_material::{BodyMaterial};
use crate::model::body_part::{BodyPart, PartType};

pub const MAX_HISTORY_COUNT: usize = 100;

#[derive(Component, Clone)]
pub struct PreviousBodyMaterial(pub BodyMaterial);

#[derive(Component, Clone)]
pub struct PreviousTransform(pub Transform);

#[derive(Component, Clone)]
pub struct PreviousPartType(pub PartType);

#[derive(Clone)]
pub struct BodyPartSnapshot {
    pub part: BodyPart,
    pub material: BodyMaterial,
    pub transform: Transform,
    pub parent: Option<Entity>,
    pub children: Vec<BodyPartSnapshot>,
    pub restored_entity: Option<Entity>
}

pub enum Action {
    MaterialEdit {
        entity: Entity,
        old: BodyMaterial, 
        new: BodyMaterial
    },
    TransformEdit {
        entity: Entity,
        old: Transform,
        new: Transform
    },
    ResizePartType {
        entity: Entity,
        old: PartType,
        new: PartType
    },
    DeletePart {
        snapshot: BodyPartSnapshot
    },
    AssignParentEntity {
        entity: Entity,
        entity_id: u32,
       
        old_parent: Option<Entity>,
        old_parent_id: Option<u32>,
        old_transform: Transform,

        new_parent: Option<Entity>,
        new_parent_id: Option<u32>,
        new_transform: Transform
    }
}

// Register as bevy resource
#[derive(Resource, Default)]
pub struct EditHistory {
    pub undo_stacks: Vec<Action>,
    pub redo_stacks: Vec<Action>,
    pub restoring: bool
}

#[derive(SystemParam)]
pub struct BodyContext<'w, 's> {
    transform_query: Query<'w, 's, &'static mut Transform>,
    body_material_query: Query<'w, 's, &'static mut BodyMaterial>,
    body_part_query: Query<'w, 's, &'static mut BodyPart>,
    part_mesh_query: Query<'w, 's, &'static Mesh3d>,
    meshes: ResMut<'w, Assets<Mesh>>,
    hierarychy: ResMut<'w, BodyHierarchy>
}

pub struct EditHistoryManager;

/*
 * Responsible to track if changes is made on monster body material
 * Responsible to undo changes (reverse changes)
 * Responsible to redo (reversed the undo)
 * */
impl EditHistoryManager {
    pub fn handle_record(
        changed_body_material_query: Query<(Entity, &BodyMaterial, &PreviousBodyMaterial), Changed<BodyMaterial>>,
        mut history: ResMut<EditHistory>,
        mut commands: Commands
    ) {
        // if undd or redo is restoring changes 
        if history.restoring {return;};
        
        // Otherwise, start putting changes to history undo stacks 
        // and clear redo action stacks 
        for (entity, current_body_material, previous_body_material) in &changed_body_material_query {
            let is_changed = *current_body_material != previous_body_material.0;
            if !is_changed {continue;};
            history.undo_stacks.push(Action::MaterialEdit{
                entity, 
                old: previous_body_material.0.clone(),
                new: current_body_material.clone()
            });
            if history.undo_stacks.len() > MAX_HISTORY_COUNT {history.undo_stacks.remove(0);};
            history.redo_stacks.clear();
            commands.entity(entity).insert(PreviousBodyMaterial(current_body_material.clone()));
        }
    }

    pub fn handle_undo(
        keys: Res<ButtonInput<KeyCode>>,
        mut body_ctx: BodyContext,
        mut history: ResMut<EditHistory>,
        mut commands: Commands, 
        mut materials: ResMut<Assets<StandardMaterial>>,
    ){
        let is_combination_pressed: bool = (Self::is_ctrl_pressed(&keys) || Self::is_cmd_pressed(&keys)) 
            && !Self::is_shift_pressed(&keys)
            && Self::is_z_pressed(&keys);

        if !is_combination_pressed {return;};

        let Some(mut action) = history.undo_stacks.pop() else {return;};

        match action {
            Action::MaterialEdit { entity, ref old, .. } => {
                let Ok(mut body_material) = body_ctx.body_material_query.get_mut(entity) else {return;};
                history.restoring = true;
        
                // applying old material to current material as a whole
                *body_material = old.clone();
    
                // queue to register previous body material
                commands.entity(entity)
                    .insert(PreviousBodyMaterial(old.clone()));

                history.restoring = false;
            },
            Action::TransformEdit { entity, old, .. } => {
                let Ok(mut transform) = body_ctx.transform_query.get_mut(entity) else {return;};
                history.restoring = true;
                *transform = old;
                history.restoring = false;
            },
            Action::DeletePart { ref mut snapshot } => {
                history.restoring = true;
                let resolved_parent = snapshot.parent.
                    and_then(|_| body_ctx.hierarychy.entities.get(&snapshot.part.parent_id?).copied());
                Self::restore_snapshot(snapshot, resolved_parent, &mut commands, &mut body_ctx.meshes, &mut materials, &mut body_ctx.hierarychy);
                history.restoring = false;
            },
            Action::ResizePartType { entity, ref old, .. } => {
                let Ok(mesh3d) = body_ctx.part_mesh_query.get(entity) else {return;};
                let Ok(mut body_part) = body_ctx.body_part_query.get_mut(entity) else {return;};
                let Some(mesh) = body_ctx.meshes.get_mut(&mesh3d.0) else {return;};

                history.restoring = true;
                body_part.part_type = old.clone();         
                *mesh = body_part.part_type.build_mesh();
                history.restoring = false;
            },
            Action::AssignParentEntity { entity_id, old_parent_id, old_transform, .. } => {
                let Some(&entity) = body_ctx.hierarychy.entities.get(&entity_id) else {return;};
                if let Ok(mut transform) = body_ctx.transform_query.get_mut(entity) {
                    *transform = old_transform;
                }
                match old_parent_id {
                    Some(parent_id) => {
                        if let Some(&parent) = body_ctx.hierarychy.entities.get(&parent_id) {
                            commands.entity(parent).add_child(entity);
                        } 
                    },
                    None => {
                        commands.entity(entity).remove::<ChildOf>();
                    }
                }
            },
        };
          
        // save to redo stack so this undo can be reversed
        history.redo_stacks.push(action);
    }

    pub fn handle_redo(
        keys: Res<ButtonInput<KeyCode>>,
        mut body_ctx: BodyContext, 
        mut history: ResMut<EditHistory>,
        mut commands: Commands, 
    ) {
        let is_combination_pressed: bool = (Self::is_ctrl_pressed(&keys) || Self::is_cmd_pressed(&keys))
            && Self::is_shift_pressed(&keys)
            && Self::is_z_pressed(&keys);

        if !is_combination_pressed {return;}

        let Some(mut action) = history.redo_stacks.pop() else {return;};
     
        match action {
            Action::MaterialEdit { entity, ref new, .. } => { 
                let Ok(mut body_material) = body_ctx.body_material_query.get_mut(entity) else {return;};
                history.restoring = true;
                *body_material = new.clone();
                commands.entity(entity)
                    .insert(PreviousBodyMaterial(new.clone()));
                history.restoring = false;
            },
            Action::TransformEdit { entity, new, .. } => {
                let Ok(mut transform) = body_ctx.transform_query.get_mut(entity) else {return;};
                history.restoring = true;
                *transform = new;
                history.restoring = false;
            },
            Action::DeletePart { ref mut snapshot } => {
                let Some(entity) = snapshot.restored_entity else {return;};
                history.restoring = true;
                body_ctx.hierarychy.entities.remove(&snapshot.part.id);
                commands.entity(entity).despawn();
                history.restoring = false;
            },
            Action::ResizePartType { entity, ref new, .. } => {
                let Ok(mesh3d) = body_ctx.part_mesh_query.get(entity) else {return;};
                let Ok(mut body_part) = body_ctx.body_part_query.get_mut(entity) else {return;};
                let Some(mesh) = body_ctx.meshes.get_mut(&mesh3d.0) else {return;};
                history.restoring = true;
                body_part.part_type = new.clone();
                *mesh = body_part.part_type.build_mesh();
                history.restoring = false;
            },
            Action::AssignParentEntity { entity_id, new_parent_id, new_transform, .. } => {
                let Some(&entity) = body_ctx.hierarychy.entities.get(&entity_id) else {return;};
                if let Ok(mut transform) = body_ctx.transform_query.get_mut(entity) {
                    *transform = new_transform;
                }

                match new_parent_id {
                    Some(parent_id) => {
                        if let Some(&parent) = body_ctx.hierarychy.entities.get(&parent_id) {
                            commands.entity(parent).add_child(entity);
                        }
                    },
                    None => {
                        commands.entity(entity).remove::<ChildOf>();
                    },
                }
            },
        }
                
        // save to undo stack so this redo can be reversed
        history.undo_stacks.push(action);
    }

    fn restore_snapshot(
        body_part_snapshot: &mut BodyPartSnapshot,
        parent: Option<Entity>,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        hierarchy: &mut BodyHierarchy
    ) {
        let mesh = body_part_snapshot.part.part_type.build_mesh();
        let material_handle = materials.add(StandardMaterial{
            base_color: body_part_snapshot.material.base_color,
            perceptual_roughness: body_part_snapshot.material.roughness,
            metallic: body_part_snapshot.material.metallic,
            ..default()
        });

        let entity = commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material_handle.clone()),
            body_part_snapshot.transform,
            body_part_snapshot.part.clone(),
            OriginalMaterial(material_handle.clone()),
            body_part_snapshot.material.clone(),
            PreviousBodyMaterial(body_part_snapshot.material.clone()) 
        )).observe(Selector::on_press).id();

        body_part_snapshot.restored_entity = Some(entity);
        hierarchy.entities.insert(body_part_snapshot.part.id, entity);
        
        if let Some(parent_entity) = parent {
            commands.entity(parent_entity).add_child(entity);
        }
         
        for child in &mut body_part_snapshot.children {
            Self::restore_snapshot(child, Some(entity), commands, meshes, materials, hierarchy);
        }
    }

    fn is_ctrl_pressed(keys: &Res<ButtonInput<KeyCode>>) -> bool {
        keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight)
    }

    fn is_cmd_pressed(keys: &Res<ButtonInput<KeyCode>>) -> bool {
        keys.pressed(KeyCode::SuperLeft) || keys.pressed(KeyCode::SuperRight)
    }

    fn is_shift_pressed(keys: &Res<ButtonInput<KeyCode>>) -> bool {
        keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)
    }
    
    fn is_z_pressed(keys: &Res<ButtonInput<KeyCode>>) -> bool {
        keys.just_pressed(KeyCode::KeyZ)
    }

}
