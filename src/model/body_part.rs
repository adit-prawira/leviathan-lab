use core::fmt;

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

impl fmt::Display for PartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartType::Sphere { .. } => write!(f, "Sphere"),
            PartType::Capsule { .. } => write!(f, "Capsule")
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

pub struct BodyPartBuilder {
    id: u32,
    name: String, 
    part_type: PartType, 
    position: Vec3
}

impl BodyPartBuilder {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: String::from("Body"),
            part_type: PartType::Sphere { radius: 0.5 },
            position: Vec3::ZERO
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn part_type(mut self, part_type: PartType) -> Self {
        self.part_type = part_type;
        self
    }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn build(self) -> BodyPart {
         BodyPart {
             id: self.id,
             name: self.name,
             part_type: self.part_type,
             parent_id: None,
             children: vec![],
             translation: self.position.to_array(),
             rotation: [0.0, 0.0, 0.0, 1.0],
             scale: [1.0, 1.0, 1.0],
             material_id: 0,
         }
     }
}
