use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use core::fmt;

use crate::history::edit_history::PendingSculptChanges;
use crate::model::body_material::BodyMaterial;
use crate::model::body_part::BodyPart;

use super::bvh::BvhCache;
use super::selector::Selection;
use super::symmetry::SymmetryMode;

pub const INITIAL_BODY_PART_COLOR: Color = Color::srgba(0.5, 0.8, 0.5, 1.0);
pub const INITIAL_METALLIC_COEFFICIENT: f32 = 0.0;
pub const INITIAL_ROUGHNESS_COEFFICIENT: f32 = 0.0;

#[derive(Resource, Default, PartialEq)]
pub enum SculptMode {
    #[default]
    Select,
    AddBodyPart,
    Sculpt
}

impl fmt::Display for SculptMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SculptMode::AddBodyPart => write!(f, "Add Body Part"),
            SculptMode::Select => write!(f, "Select"),
            SculptMode::Sculpt => write!(f, "Sculpt")
        }
    }
}

#[derive(Resource, Default, PartialEq)]
pub enum SculptBodyPartType {
    #[default]
    Sphere, 
    Capsule,
    Cone,
    Torus, 
    Cylinder
}

#[derive(Resource, Default, PartialEq, Clone)]
pub enum BrushMode {
    #[default]
    Pull,
    Push, 
    Smooth, 
    Flatten
}

impl fmt::Display for BrushMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrushMode::Pull => write!(f, "Pull"),
            BrushMode::Push => write!(f, "Push"),
            BrushMode::Smooth => write!(f, "Smooth"),
            BrushMode::Flatten => write!(f, "Flatten"),
        }
    }
}

#[derive(Resource)]
pub struct SculptBrush {
    pub radius: f32,
    pub strength: f32,
    pub mode: BrushMode
}

impl Default for SculptBrush {
    fn default() -> Self {
        Self {radius: 0.5, strength: 0.3, mode: BrushMode::Pull}
    }
}

impl SculptBrush {
    pub fn effective_strength(&self) -> f32 {
        self.strength * (self.radius / 0.5).min(1.0)
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

pub trait IdGenerator {
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
pub struct SculptContext<'w>{
    pub mode: Res<'w, SculptMode>,
    pub symmetry_mode: Res<'w, SymmetryMode>,
    pub brush: Res<'w, SculptBrush>,
    pub bvh_cache: ResMut<'w, BvhCache>,
    pub pending_sculpt_changes: ResMut<'w, PendingSculptChanges>,
    pub pending_sculpt_reset: ResMut<'w, PendingSculptReset>
}

#[derive(SystemParam)]
pub struct TransformContext<'w, 's> {
    pub global_transform_query: Query<'w, 's, &'static GlobalTransform>,
    pub transform_query: Query<'w, 's, &'static mut Transform>
}

#[derive(SystemParam)]
pub struct SpawnContext<'w, 's>{
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub mesh3d_query: Query<'w, 's, &'static Mesh3d>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub commands: Commands<'w, 's>,
    pub body_part_id: ResMut<'w, BodyPartId>,
    pub body_part_query: Query<'w, 's, &'static mut BodyPart>
}

#[derive(SystemParam)]
pub struct ControlContext<'w> {
    pub mouse_buttons: Res<'w, ButtonInput<MouseButton>>,
    pub selection: Res<'w, Selection>,
    pub keys: Res<'w, ButtonInput<KeyCode>>
}

#[derive(SystemParam)]
pub struct SceneContext<'w, 's> {
    pub window_query: Query<'w, 's, &'static Window>,
    pub camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform)>,
    pub global_transform_query: Query<'w, 's, &'static GlobalTransform>
}

#[derive(SystemParam)]
pub struct BodyContext<'w, 's>{
    pub body_part_query: Query<'w, 's, &'static BodyPart>,
    pub body_material_query: Query<'w,'s, &'static BodyMaterial>,
    pub transform_query: Query<'w, 's, &'static Transform>,
    pub all_body_part_query: Query<'w, 's, Entity, With<BodyPart>>
}

#[derive(Resource, Default)]
pub struct PendingResize {
    pub entity: Option<Entity>,
    pub radius: Option<f32>,
    pub major_radius: Option<f32>,
    pub minor_radius: Option<f32>,
    pub height: Option<f32>,
    pub half_height: Option<f32>,
    pub half_length: Option<f32>,
    pub subdivisions: Option<u32>,
}

#[derive(Resource, Default)]
pub struct PendingSculptReset {
    pub entity: Option<Entity>
}
