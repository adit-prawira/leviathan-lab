use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::editor::resource::{SculptBodyPartType, SculptMode};

pub struct Toolbar;

impl Toolbar {
    pub fn show(
        mut mode: ResMut<SculptMode>,
        mut added_body_part_type: ResMut<SculptBodyPartType>,
        mut contexts: EguiContexts
    ) {
        // Sculpt tool top tool bar 
        egui::TopBottomPanel::top("sculpt-toolbar")
            .show(contexts.ctx_mut().expect("egui context to be available"), |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut *mode, SculptMode::Select, "Select (Esc)");
                    ui.selectable_value(&mut *mode, SculptMode::AddBodyPart, "Add Body Part (A)");
                    ui.selectable_value(&mut *mode, SculptMode::Sculpt, "Sculpt Body Part (V)");
                    if *mode != SculptMode::AddBodyPart {return;};

                    ui.separator();
                    ui.selectable_value(&mut *added_body_part_type, SculptBodyPartType::Sphere, "🟢 Sphere");
                    ui.selectable_value(&mut *added_body_part_type, SculptBodyPartType::Capsule, "💊 Capsule");
                    ui.selectable_value(&mut *added_body_part_type, SculptBodyPartType::Cone, "🔺Cone");
                    ui.selectable_value(&mut *added_body_part_type, SculptBodyPartType::Torus, "🍩 Torus");
                    ui.selectable_value(&mut *added_body_part_type, SculptBodyPartType::Cylinder, "🥫 Cylinder");
                });
        });
    }
}
