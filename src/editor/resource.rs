use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use core::fmt;

pub const INITIAL_BODY_PART_COLOR: Color = Color::srgba(0.5, 0.8, 0.5, 1.0);
pub const INITIAL_METALLIC_COEFFICIENT: f32 = 0.0;
pub const INITIAL_ROUGHNESS_COEFFICIENT: f32 = 0.0;

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
pub enum SculptBodyPartType {
    #[default]
    Sphere, 
    Capsule,
    Cone,
    Torus, 
    Cylinder
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
pub struct TransformContext<'w, 's> {
    pub global_transform_query: Query<'w, 's, &'static GlobalTransform>,
    pub transform_query: Query<'w, 's, &'static mut Transform>
}
