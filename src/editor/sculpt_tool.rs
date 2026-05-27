use bevy::ecs::relationship::Relationship;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::editor::selector::{OriginalMaterial, Selector};
use crate::history::edit_history::{Action, BodyPartSnapshot, EditHistory, MAX_HISTORY_COUNT, PreviousBodyMaterial};
use crate::model::body_material::BodyMaterial;
use crate::model::body_part::{BodyPart, BodyPartBuilder, PartType};

use super::resource::{BodyPartId, INITIAL_BODY_PART_COLOR, INITIAL_METALLIC_COEFFICIENT, INITIAL_ROUGHNESS_COEFFICIENT, IdGenerator, SculptBodyPartType, SculptMode};
use super::selector::Selection;

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
    window_query: Query<'w, 's, &'static Window>,
    camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform)>,
    global_transform_query: Query<'w, 's, &'static GlobalTransform>
}

#[derive(SystemParam)]
pub struct BodyContext<'w, 's>{
    body_part_query: Query<'w, 's, &'static BodyPart>,
    body_material_query: Query<'w,'s, &'static BodyMaterial>,
    transform_query: Query<'w, 's, &'static Transform>,
    all_body_part_query: Query<'w, 's, Entity, With<BodyPart>>
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
        added_body_part_type: Res<SculptBodyPartType>,
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
        
        let Ok(window) = scene_ctx.window_query.single() else {return;};
        let Some(cursor_position) = window.cursor_position() else {return;};
        
        let Ok((camera, camera_global_transform)) = scene_ctx.camera_query.single() else {return;};
        let Ok(ray_3d) = camera.viewport_to_world(camera_global_transform, cursor_position) else {return;};
        
        let Some(distance) = ray_3d.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {return;};
        let spawn_position = ray_3d.get_point(distance);

        let (mesh, body_part_type) = match *added_body_part_type {
            SculptBodyPartType::Sphere => {
                let part_type = PartType::Sphere { radius: 0.5 };
                (
                    part_type.build_mesh(), 
                    part_type 
                )
            },
            SculptBodyPartType::Capsule => {
                let part_type = PartType::Capsule { radius: 0.3, half_length: 0.5 };
                (part_type.build_mesh(), part_type)
            } 
        };
        
        let body_part_id = spawn_ctx.body_part_id.next();
        let material_handle = Self::build_material_handle(&mut spawn_ctx.materials);
        let body_material = Self::build_body_material();
        let previous_body_material = PreviousBodyMaterial(body_material.clone());
        let body_part = BodyPartBuilder::new(body_part_id)
            .name(body_part_type.to_string())
            .part_type(body_part_type)
            .position(spawn_position)
            .build();
 
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

        if let Some(parent_entity) = control_ctx.selection.entity {
            if let Ok(parent_world) = scene_ctx.global_transform_query.get(parent_entity) {
                let local_spawn_matrix = Mat4::from(parent_world.affine().inverse()) * Mat4::from_translation(spawn_position);
                let local_spawn_transform = Transform::from_matrix(local_spawn_matrix);
                spawn_ctx.commands.entity(child_entity).insert(local_spawn_transform);
            };
            spawn_ctx.commands.entity(parent_entity).add_child(child_entity);
        };
    }
    
    pub fn handle_button_input(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<SculptMode>) {
        if keys.just_pressed(KeyCode::KeyA) {*mode = SculptMode::AddBodyPart;};
        if keys.just_pressed(KeyCode::Escape) {*mode = SculptMode::Select;};
    }

    pub fn handle_delete_body_part(
        keys: Res<ButtonInput<KeyCode>>, 
        body_ctx: BodyContext, 
        parent_query: Query<&ChildOf>,
        children_query: Query<&Children>, 
        mut selection: ResMut<Selection>,
        mut history: ResMut<EditHistory>,
        mut commands: Commands
    ) {
        let is_delete_command_pressed = keys.just_pressed(KeyCode::Delete)
            || keys.just_pressed(KeyCode::Backspace);
        if !is_delete_command_pressed {return;};

        let Some(entity) = selection.entity else {return;};
        
        // ensure at least 1 part exist 
        if body_ctx.all_body_part_query.iter().len() <= 1 {return;}

        let parent = parent_query.get(entity).ok().map(|p| p.get()); 
        let snapshot = Self::capture_snapshot(entity, parent, &children_query, &body_ctx);
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
        let part = body_ctx.body_part_query.get(entity).unwrap().clone();
        let material = body_ctx.body_material_query.get(entity).unwrap().clone();
        let transform = *body_ctx.transform_query.get(entity).unwrap();
        let children = if let Ok(kids) = children.get(entity) {
            kids.iter().filter(|&k| body_ctx.body_part_query.contains(k))
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
}
