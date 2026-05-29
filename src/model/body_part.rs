use core::fmt;

use bevy::prelude::*;

pub const DEFAULT_SUBDIVISION: u32 = 2;
pub const DEFAULT_RESOLUTION: u32 = 32;

#[derive(Clone, Debug, Component, PartialEq)]
pub enum PartType {
    Sphere { radius: f32 },
    Capsule { radius: f32, half_length: f32 },
    Cone {radius: f32, height: f32},
    Torus {major_radius: f32, minor_radius: f32},
    Cylinder {radius:f32, half_height: f32}
}

impl PartType {
    pub fn build_mesh(&self) -> Mesh {
        self.build_mesh_with(DEFAULT_SUBDIVISION)
    }

    pub fn build_mesh_with(&self, subdivisions: u32) -> Mesh{
        match self {
            Self::Sphere { radius } => Sphere::new(*radius).mesh()
                .ico(subdivisions)
                .expect("subdivision to be in range of 1 - 5"),
            Self::Capsule { radius, half_length } => Capsule3d{radius: *radius, half_length: *half_length}.mesh()
                .rings(subdivisions)
                .build(), 
            Self::Cone { radius, height} => Cone{radius: *radius, height: *height}.mesh()
                .resolution(DEFAULT_RESOLUTION) 
                .build(),
            Self::Torus { major_radius, minor_radius } => Torus{major_radius: *major_radius, minor_radius: *minor_radius}.mesh()
                .major_resolution(DEFAULT_RESOLUTION.try_into().unwrap())
                .minor_resolution(DEFAULT_RESOLUTION.try_into().unwrap())
                .build(),
            Self::Cylinder { radius, half_height } => Cylinder{radius: *radius, half_height: *half_height}.mesh()
                .resolution(DEFAULT_RESOLUTION)
                .build(),
        }
    }
}

impl fmt::Display for PartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartType::Sphere { .. } => write!(f, "Sphere"),
            PartType::Capsule { .. } => write!(f, "Capsule"),
            PartType::Cone { .. } => write!(f, "Cone"),
            PartType::Torus { .. } => write!(f, "Torus"),
            PartType::Cylinder { .. } => write!(f, "Cylinder"),
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
    pub subdivisions: u32
}

impl BodyPart {
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    pub fn build_mesh(&self) -> Mesh {
        self.part_type.build_mesh_with(self.subdivisions)
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
             subdivisions: DEFAULT_SUBDIVISION
         }
     }
}
