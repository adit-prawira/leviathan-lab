use core::fmt;

use bevy::ecs::relationship::Relationship;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::editor::selector::{OriginalMaterial, Selector};
use crate::history::edit_history::{Action, BodyPartSnapshot, EditHistory, MAX_HISTORY_COUNT, PreviousBodyMaterial};
use crate::model::body_material::BodyMaterial;
use crate::model::body_part::{BodyPart, PartType};

use super::selector::Selection;

const INITIAL_BODY_PART_COLOR: Color = Color::srgba(0.5, 0.8, 0.5, 1.0);
const INITIAL_METALLIC_COEFFICIENT: f32 = 0.0;
const INITIAL_ROUGHNESS_COEFFICIENT: f32 = 0.0;

#[derive(Resource, Default, PartialEq)]
pub enum SculptMode {
    #[default]
    Select,
    AddBodyPart
}

impl fmt::Display for SculptMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SculptMode::AddBodyPart => write!(f, "Add Body Part"),
            SculptMode::Select => write!(f, "Select")
        }
    }
}

#[derive(Resource, Default, PartialEq)]
pub enum BodyPartType {
    #[default]
    Sphere, 
    Capsule
}

impl fmt::Display for BodyPartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyPartType::Capsule => write!(f, "Capsule"),
            BodyPartType::Sphere => write!(f, "Sphere")
        }
    }
}

#[derive(Resource)]
pub struct BodyPartId(pub u32);

impl Default for BodyPartId {
    // new body part id will 
    // start with 100 
    fn default() -> Self {
        Self(100)
    }
}

trait IdGenerator {
    fn next(&mut self) -> u32;
}

impl IdGenerator for BodyPartId {
    fn next(&mut self) -> u32 {
        let id = self.0;
        self.0 += 1;
        id
    }
}

#[derive(SystemParam)]
pub struct SpawnContext<'w, 's>{
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
    commands: Commands<'w, 's>,
    body_part_id: ResMut<'w, BodyPartId>
}

#[derive(SystemParam)]
pub struct ControlContext<'w> {
    buttons: Res<'w, ButtonInput<MouseButton>>,
    selection: Res<'w, Selection>
}

#[derive(SystemParam)]
pub struct SceneContext<'w, 's> {
    windows: Query<'w, 's, &'static Window>,
    cameras: Query<'w, 's, (&'static Camera, &'static GlobalTransform)>
}

#[derive(SystemParam)]
pub struct BodyContext<'w, 's>{
    parts: Query<'w, 's, &'static BodyPart>,
    materials: Query<'w,'s, &'static BodyMaterial>,
    transforms: Query<'w, 's, &'static Transform>,
    all_parts: Query<'w, 's, Entity, With<BodyPart>>
}

#[derive(Resource, Default)]
pub struct PendingResize {
    pub entity: Option<Entity>,
    pub radius: Option<f32>,
    pub half_length: Option<f32>
}

pub struct SculptTool;

impl SculptTool {
    pub fn handle_resize( 
        part_mesh_query: Query<&Mesh3d>,
        mut pending_resize: ResMut<PendingResize>,
        mut body_part_query: Query<&mut BodyPart>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut history: ResMut<EditHistory>
    ) {
        let Some(entity) = pending_resize.entity else {return;}; 
        let Ok(mesh3d) = part_mesh_query.get(entity) else {return;};

        let Ok(mut body_part) = body_part_query.get_mut(entity) else {return;};
        let old_part_type = body_part.part_type.clone();

        match &mut body_part.part_type {
            PartType::Sphere { radius } => {
                if let Some(pending_radius) = pending_resize.radius {
                    *radius = pending_radius;
                }
            },
            PartType::Capsule { radius, half_length } => {
                if let Some(pending_radius) = pending_resize.radius {
                    *radius = pending_radius;
                }
                
                if let Some(pending_half_resize) = pending_resize.half_length {
                    *half_length = pending_half_resize;
                }
            }
        }

        let new_part_type = body_part.part_type.clone();
        let Some(mesh) = meshes.get_mut(&mesh3d.0) else {return;};
        
        let new_mesh = body_part.part_type.build_mesh(); 
        
        *mesh = new_mesh;
        *pending_resize = PendingResize::default();
        
        let part_type_changed = old_part_type != new_part_type;
        if !part_type_changed {return;};

        history.undo_stacks.push(Action::ResizePartType { 
            entity, 
            old: old_part_type, 
            new: new_part_type 
        });
        history.redo_stacks.clear();
    }

    pub fn handle_add_body_part(
        mode: Res<SculptMode>,
        added_body_part_type: Res<BodyPartType>,
        control_ctx: ControlContext, 
        scene_ctx: SceneContext, 
        mut egui_contexts: EguiContexts,
        mut spawn_ctx: SpawnContext
    ) {
        if *mode != SculptMode::AddBodyPart {return;};
        if !control_ctx.buttons.just_pressed(MouseButton::Left) {return;};

        let is_mouse_pointer_touching_properties_panel = egui_contexts.ctx_mut()
            .expect("egui context to be available").wants_pointer_input();  
        if is_mouse_pointer_touching_properties_panel {return;};
        
        let Ok(window) = scene_ctx.windows.single() else {return;};
        let Some(cursor_position) = window.cursor_position() else {return;};
        
        let Ok((camera, camera_global_transform)) = scene_ctx.cameras.single() else {return;};
        let Ok(ray_3d) = camera.viewport_to_world(camera_global_transform, cursor_position) else {return;};
        
        let Some(distance) = ray_3d.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {return;};
        let spawn_position = ray_3d.get_point(distance);

        let (mesh, body_part_type, body_part_name) = match *added_body_part_type {
            BodyPartType::Sphere => (
                Sphere::new(0.5).mesh().build(), 
                PartType::Sphere { radius: 0.5 }, 
                "Sphere"
            ),
            BodyPartType::Capsule => (
                Capsule3d{radius: 0.3, half_length: 0.5}.mesh().build(), 
                PartType::Capsule { radius: 0.3, half_length: 0.5 }, 
                "Capsule"
            )
        };
        
        let body_part_id = spawn_ctx.body_part_id.next();
        let material_handle = Self::build_material_handle(&mut spawn_ctx.materials);
        let body_material = Self::build_body_material();
        let previous_body_material = PreviousBodyMaterial(body_material.clone());
        let body_part = Self::build_body_part(body_part_id, body_part_name.to_string(), body_part_type, spawn_position);
        
        let child_entity = spawn_ctx.commands.spawn((
            Mesh3d(spawn_ctx.meshes.add(mesh)),
            MeshMaterial3d(material_handle.clone()),
            Transform::from_translation(spawn_position),
            body_part.clone(),
            OriginalMaterial(material_handle),
            body_material,
            previous_body_material
        ))
        .observe(Selector::on_press)
        .id();

        let Some(parent_entity) = control_ctx.selection.entity else {return;};
        spawn_ctx.commands.entity(parent_entity).add_child(child_entity);
    }
    
    pub fn handle_button_input(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<SculptMode>) {
        if keys.just_pressed(KeyCode::KeyA) {*mode = SculptMode::AddBodyPart;};
        if keys.just_pressed(KeyCode::Escape) {*mode = SculptMode::Select;};
    }

    pub fn handle_delete_body_part(
        keys: Res<ButtonInput<KeyCode>>, 
        body_ctx: BodyContext, 
        parents: Query<&ChildOf>,
        children: Query<&Children>, 
        mut selection: ResMut<Selection>,
        mut history: ResMut<EditHistory>,
        mut commands: Commands
    ) {
        let is_delete_command_pressed = keys.just_pressed(KeyCode::Delete)
            || keys.just_pressed(KeyCode::Backspace);
        if !is_delete_command_pressed {return;};

        let Some(entity) = selection.entity else {return;};
        
        // ensure at least 1 part exist 
        if body_ctx.all_parts.iter().len() <= 1 {return;}

        let parent = parents.get(entity).ok().map(|p| p.get()); 
        let snapshot = Self::capture_snapshot(entity, parent, &children, &body_ctx);
        history.undo_stacks.push(Action::DeletePart { snapshot });
        if history.undo_stacks.len() > MAX_HISTORY_COUNT {history.undo_stacks.remove(0);};
        history.redo_stacks.clear();
        selection.entity = None;
        commands.entity(entity).despawn();
    }

    fn capture_snapshot(
        entity: Entity, 
        parent: Option<Entity>,
        children: &Query<&Children>,
        body_ctx: &BodyContext,
    ) -> BodyPartSnapshot {
        let part = body_ctx.parts.get(entity).unwrap().clone();
        let material = body_ctx.materials.get(entity).unwrap().clone();
        let transform = *body_ctx.transforms.get(entity).unwrap();
        let children = if let Ok(kids) = children.get(entity) {
            kids.iter().filter(|&k| body_ctx.parts.contains(k))
                .map(|k| Self::capture_snapshot(k, Some(entity), children, body_ctx))
                .collect()
        }else {
            vec![]
        };
        BodyPartSnapshot { 
            part, 
            material, 
            transform, 
            parent, 
            children,
            restored_entity: None
        }
    }

    fn build_material_handle(materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: INITIAL_BODY_PART_COLOR,
            perceptual_roughness: INITIAL_ROUGHNESS_COEFFICIENT,
            metallic: INITIAL_METALLIC_COEFFICIENT,
            ..default()
        })
    }

    fn build_body_material() -> BodyMaterial {
        BodyMaterial {
            base_color: INITIAL_BODY_PART_COLOR,
            roughness: INITIAL_ROUGHNESS_COEFFICIENT,
            metallic: INITIAL_METALLIC_COEFFICIENT,
        }
    }

    fn build_body_part(id: u32, name: String, part_type: PartType, position: Vec3) -> BodyPart {
        BodyPart {
            id,
            name,
            part_type,
            parent_id: None,
            children: vec![],
            translation: position.to_array(),
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
            material_id: 0,
        }
    }
}



