use bevy::prelude::*;

#[derive(Clone, Debug, Component, PartialEq)]
pub enum PartType {
    Sphere { radius: f32 },
    Capsule { radius: f32, half_length: f32 },
}

impl PartType {
    pub fn build_mesh(&self) -> Mesh{
        match self {
            Self::Sphere { radius } => Sphere::new(*radius).mesh().build(),
            Self::Capsule { radius, half_length } => Capsule3d{radius: *radius, half_length: *half_length}.mesh().build()
        }
    }
}

#[derive(Clone, Debug, Component, PartialEq)]
pub struct BodyPart {
    pub id: u32,
    pub name: String,
    pub part_type: PartType,
    pub parent_id: Option<u32>,
    pub children: Vec<u32>,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
    pub material_id: u32,
}

impl BodyPart {
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
}
