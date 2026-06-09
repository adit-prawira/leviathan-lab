use std::f32::consts::FRAC_1_SQRT_2;

use crate::model::body_part::{BodyPart, DEFAULT_SUBDIVISION, PartType};
use crate::model::material::MaterialData;

pub struct Monster {
    pub parts: Vec<BodyPart>,
    pub materials: Vec<MaterialData>,
}

impl Monster {
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn default_hatchling() -> Self {
        let mut monster: Monster = Self::new();
        let skin_material: MaterialData = MaterialData {
            base_color: [0.15, 0.45, 0.3, 1.0],
            roughness: 0.6,
            metallic: 0.05,
        };
        let belly_material: MaterialData = MaterialData {
            base_color: [0.7, 0.75, 0.7, 1.0],
            roughness: 0.7,
            metallic: 0.0,
        };
        let eye_material: MaterialData = MaterialData {
            base_color: [0.0, 0.0, 0.0, 1.0],
            roughness: 0.1,
            metallic: 0.0,
        };

        monster.materials.extend([skin_material, belly_material, eye_material]);
        monster.parts.extend([
            monster.build_body(),
            monster.build_head(),
            monster.build_tail(),
            monster.build_belly(),
            monster.build_dorsal_fin(),
            monster.build_left_eye(),
            monster.build_right_eye(),
        ]);
        monster
    }

    fn build_body(&self) -> BodyPart {
        BodyPart {
            id: 0,
            name: "Body".into(),
            part_type: PartType::Sphere { radius: 0.8 },
            parent_id: None,
            children: vec![1, 2, 3, 4],
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 0.7, 1.5],
            material_id: 0,
            subdivisions: DEFAULT_SUBDIVISION,
            is_sculpted: false
        }
    }

    fn build_head(&self) -> BodyPart {
        BodyPart {
            id: 1,
            name: "Head".into(),
            part_type: PartType::Sphere { radius: 0.35 },
            parent_id: Some(0),
            children: vec![5, 6],
            translation: [0.0, 0.0, -0.8],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
            material_id: 0,
            subdivisions: DEFAULT_SUBDIVISION,
            is_sculpted: false
        }
    }

    fn build_tail(&self) -> BodyPart {
        BodyPart {
            id: 2,
            name: "Tail".into(),
            part_type: PartType::Capsule { radius: 0.3, half_length: 0.2 },
            parent_id: Some(0),
            children: vec![],
            translation: [0.0, 0.0, 1.0],
            rotation: [FRAC_1_SQRT_2, 0.0, 0.0, FRAC_1_SQRT_2],
            scale: [0.7, 0.7, 0.7],
            material_id: 0,
            subdivisions: DEFAULT_SUBDIVISION,
            is_sculpted: false
        }
    }

    fn build_belly(&self) -> BodyPart {
        BodyPart {
            id: 3,
            name: "Belly".into(),
            part_type: PartType::Sphere { radius: 0.2 },
            parent_id: Some(0),
            children: vec![],
            translation: [0.0, -0.35, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [0.9, 0.4, 1.3],
            material_id: 1,
            subdivisions: DEFAULT_SUBDIVISION,
            is_sculpted: false
        }
    }

    fn build_dorsal_fin(&self) -> BodyPart {
        BodyPart {
            id: 4,
            name: "DorsalFin".into(),
            part_type: PartType::Sphere { radius: 0.25 },
            parent_id: Some(0),
            children: vec![],
            translation: [0.0, 0.5, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [0.3, 1.0, 1.0],
            material_id: 0,
            subdivisions: DEFAULT_SUBDIVISION,
            is_sculpted: false
        }
    }

    fn build_left_eye(&self) -> BodyPart {
        BodyPart {
            id: 5,
            name: "LeftEye".into(),
            part_type: PartType::Sphere { radius: 0.08 },
            parent_id: Some(1),
            children: vec![],
            translation: [-0.28, 0.16, -0.16],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
            material_id: 2,
            subdivisions: DEFAULT_SUBDIVISION,
            is_sculpted: false
        }
    }

    fn build_right_eye(&self) -> BodyPart {
        BodyPart {
            id: 6,
            name: "RightEye".into(),
            part_type: PartType::Sphere { radius: 0.08 },
            parent_id: Some(1),
            children: vec![],
            translation: [0.28, 0.16, -0.16],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
            material_id: 2,
            subdivisions: DEFAULT_SUBDIVISION,
            is_sculpted: false
        }
    }
}

impl Default for Monster {
    fn default() -> Self {
        Self::new()
    }
}
