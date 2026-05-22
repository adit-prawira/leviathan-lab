use core::fmt;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::editor::selector::{OriginalMaterial, Selector};
use crate::history::edit_history::PreviousBodyMaterial;
use crate::model::body_material::BodyMaterial;
use crate::model::body_part::{BodyPart, PartType};

use super::selector::Selection;

pub struct SculptTool;

const INITIAL_BODY_PART_COLOR: Color = Color::srgba(0.5, 0.8, 0.5, 1.0);
const INITIAL_METALLIC_COEFFICIENT: f32 = 0.0;
const INITIAL_ROUGHNESS_COEFFICIENT: f32 = 0.0;

impl SculptTool {
    pub fn on_add_body_part(
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
    
    pub fn mode_keys(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<SculptMode>) {
        if keys.just_pressed(KeyCode::KeyA) {*mode = SculptMode::AddBodyPart;};
        if keys.just_pressed(KeyCode::Escape) {*mode = SculptMode::Select;};
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

