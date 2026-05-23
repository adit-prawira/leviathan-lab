use bevy::prelude::*;
use crate::model::body_material::{BodyMaterial};

#[derive(Component, Clone)]
pub struct PreviousBodyMaterial(pub BodyMaterial);

#[derive(Component, Clone)]
pub struct PreviousTransform(pub Transform);

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
    }
}

pub const MAX_HISTORY_COUNT: usize = 100;

// Register as bevy resource
#[derive(Resource, Default)]
pub struct EditHistory {
    pub undo_stacks: Vec<Action>,
    pub redo_stacks: Vec<Action>,
    pub restoring: bool
}

pub struct EditHistoryPlugin;

/*
 * Responsible to track if changes is made on monster body material
 * Responsible to undo changes (reverse changes)
 * Responsible to redo (reversed the undo)
 * */
impl EditHistoryPlugin {
    pub fn record(
        changes: Query<(Entity, &BodyMaterial, &PreviousBodyMaterial), Changed<BodyMaterial>>,
        mut history: ResMut<EditHistory>,
        mut commands: Commands
    ) {
        // if undd or redo is restoring changes 
        if history.restoring {return;};
        
        // Otherwise, start putting changes to history undo stacks 
        // and clear redo action stacks 
        for (entity, current_body_material, previous_body_material) in &changes {
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

    pub fn undo(
        keys: Res<ButtonInput<KeyCode>>,
        mut transforms: Query<&mut Transform>,
        mut body_materials: Query<&mut BodyMaterial>,
        mut history: ResMut<EditHistory>,
        mut commands: Commands
    ){
        let is_combination_pressed: bool = (Self::is_ctrl_pressed(&keys) || Self::is_cmd_pressed(&keys)) 
            && !Self::is_shift_pressed(&keys)
            && Self::is_z_pressed(&keys);

        if !is_combination_pressed {return;};

        let Some(action) = history.undo_stacks.pop() else {return;};

        match action {
            Action::MaterialEdit { entity, ref old, .. } => {
                let Ok(mut body_material) = body_materials.get_mut(entity) else {return;};
                history.restoring = true;
        
                // applying old material to current material as a whole
                *body_material = old.clone();
                
                // queue to register previous body material
                commands.entity(entity)
                    .insert(PreviousBodyMaterial(old.clone()));

                history.restoring = false;
            },
            Action::TransformEdit { entity, old, .. } => {
                let Ok(mut transform) = transforms.get_mut(entity) else {return;};
                history.restoring = true;
                *transform = old;
                history.restoring = false;
            }
        };
          
        // save to redo stack so this undo can be reversed
        history.redo_stacks.push(action);
    }

    pub fn redo(
        keys: Res<ButtonInput<KeyCode>>,
        mut transforms: Query<&mut Transform>,
        mut body_materials: Query<&mut BodyMaterial>,
        mut history: ResMut<EditHistory>,
        mut commands: Commands
    ) {
        let is_combination_pressed: bool = (Self::is_ctrl_pressed(&keys) || Self::is_cmd_pressed(&keys))
            && Self::is_shift_pressed(&keys)
            && Self::is_z_pressed(&keys);

        if !is_combination_pressed {return;}

        let Some(action) = history.redo_stacks.pop() else {return;};
     
        match action {
            Action::MaterialEdit { entity, ref new, .. } => { 
                let Ok(mut body_material) = body_materials.get_mut(entity) else {return;};
                history.restoring = true;
                *body_material = new.clone();
                commands.entity(entity)
                    .insert(PreviousBodyMaterial(new.clone()));
                history.restoring = false;
            },
            Action::TransformEdit { entity, new, .. } => {
                let Ok(mut transform) = transforms.get_mut(entity) else {return;};
                history.restoring = true;
                *transform = new;
                history.restoring = false;
            }
        }
                
        // save to undo stack so this redo can be reversed
        history.undo_stacks.push(action);
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
