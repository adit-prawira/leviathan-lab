use std::collections::HashMap;

use crate::editor::gizmos::GizmosMode;
use crate::editor::resource::{PendingResize, PendingSculptReset, SculptBrush, SculptMode, TransformContext};
use crate::history::edit_history::{EditHistory};
use crate::model::body_hierarchy::{BodyHierarchy, EntityReference, HierarchyReference};
use crate::model::body_material::BodyMaterial;
use crate::model::body_part::{BodyPart, PartType};
use crate::editor::selector::Selection;
use crate::editor::symmetry::SymmetryMode;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::egui::{Margin, Response, Ui};
use bevy_egui::{EguiContexts, egui};

pub struct PropertiesPanel;

const MIN_SIDE_PANEL_WIDTH: f32 = 150.0;

#[derive(SystemParam)]
pub struct EditorContext<'w> {
    mode: ResMut<'w, GizmosMode>,
    symmetry_mode: ResMut<'w, SymmetryMode>,
    history: ResMut<'w, EditHistory>,
    pending_resize: ResMut<'w, PendingResize>,
    pending_sculpt_reset: ResMut<'w, PendingSculptReset>,
    sculpt_brush: ResMut<'w, SculptBrush>,
    sculpt_mode: Res<'w, SculptMode>
}

#[derive(SystemParam)]
pub struct SelectionContext<'w, 's>{
    selection: Res<'w, Selection>,
    body_part_query: Query<'w, 's, (Entity, &'static BodyPart)>,
    children_of_query: Query<'w, 's, &'static ChildOf>
}

/**
 * UI panel (Bevy UI or `bevy_egui`)
 * Shows selected part: name, position (x/y/z),
 *                      rotation (euler), scale (x/y/z)
 * Numeric fields editable (text input or drag-value)
 * Updates part transform in real time
 */
impl PropertiesPanel {
    pub fn show(
        selection_ctx: SelectionContext,
        mut contexts: EguiContexts,
        mut body_material_query: Query<&mut BodyMaterial>,
        mut commands: Commands,
        mut transform_ctx: TransformContext,
        mut editor_ctx: EditorContext
    ) {
        let Some(entity) = selection_ctx.selection.entity else { return; };
        let Ok((_, body_part)) = selection_ctx.body_part_query.get(entity) else { return; }; 
        let Ok(mut body_material) = body_material_query.get_mut(entity) else { return; };

        // render panel on the right-hand side of the screen
        egui::SidePanel::right("properties").min_width(MIN_SIDE_PANEL_WIDTH)
            .show(contexts.ctx_mut().expect("egui context to be available"),|ui| {
                Self::properties_section(ui, entity, body_part, &mut transform_ctx.transform_query);

                Self::shape_section(ui, entity, body_part, &mut editor_ctx.pending_resize, &mut editor_ctx.pending_sculpt_reset); 

                Self::material_section(ui, &mut body_material);  

                let current_mode = *editor_ctx.mode;
                let mut local_mode = current_mode;
                
                Self::mode_section(ui, &mut local_mode, &mut editor_ctx.symmetry_mode);
                
                if local_mode != current_mode {*editor_ctx.mode = local_mode;};  

                Self::hierarchy_section(
                    ui, 
                    &entity, 
                    &selection_ctx.body_part_query, 
                    &selection_ctx.children_of_query,
                    &mut transform_ctx,
                    &mut commands, 
                    &mut editor_ctx.history 
                );

                Self::sculpt_section(ui, &mut editor_ctx.sculpt_brush, &editor_ctx.sculpt_mode);
            },
        );
    }

    fn properties_section(ui: &mut Ui, entity: Entity, body_part: &BodyPart, transforms: &mut Query<&mut Transform>) {
        let Ok(mut transform) = transforms.get_mut(entity) else { return; };
        ui.heading("Properties");
        ui.separator();

        egui::Grid::new("PropertiesSection").spacing([8.0, 8.0]).show(ui, |ui| {
            ui.strong("Body Part");
            ui.label(&body_part.name);
            ui.end_row();

            ui.strong("Position");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut transform.translation.x).speed(0.01));

                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut transform.translation.y).speed(0.01));

                ui.label("Z:");
                ui.add(egui::DragValue::new(&mut transform.translation.z).speed(0.01));
            });
            ui.end_row();

            ui.strong("Rotation (Euler Degrees)");
            let (rx, ry, rz) = transform.rotation.to_euler(EulerRot::XYZ);
            let mut degrees: [f32; 3] = [rx.to_degrees(), ry.to_degrees(), rz.to_degrees()];
            let mut changed: bool = false;

            ui.horizontal(|ui| {
                ui.label("Rx:");
                let ui_rx_value: Response = ui.add(
                    egui::DragValue::new(&mut degrees[0])
                        .speed(0.01)
                        .suffix("°"),
                );
                if ui_rx_value.changed() {
                    changed = true;
                }

                ui.label("Ry:");
                let ui_ry_value: Response = ui.add(
                    egui::DragValue::new(&mut degrees[1])
                        .speed(0.01)
                        .suffix("°"),
                );
                if ui_ry_value.changed() {
                    changed = true;
                }

                ui.label("Rz:");
                let ui_rz_value: Response = ui.add(
                    egui::DragValue::new(&mut degrees[2])
                        .speed(0.01)
                        .suffix("°"),
                );
                if ui_rz_value.changed() {
                    changed = true;
                }

                if changed {
                    transform.rotation = Quat::from_euler(
                        EulerRot::XYZ,
                        degrees[0].to_radians(),
                        degrees[1].to_radians(),
                        degrees[2].to_radians(),
                    );
                }
            });
            ui.end_row();

            ui.strong("Scale");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut transform.scale.x).speed(0.01));

                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut transform.scale.y).speed(0.01));

                ui.label("Z:");
                ui.add(egui::DragValue::new(&mut transform.scale.z).speed(0.01));
            });
            ui.end_row();

            ui.strong("Shape");
            ui.horizontal(|ui| {
                ui.disable();
                let mut sphere_value = matches!(body_part.part_type, PartType::Sphere { .. });
                let mut capsule_value = matches!(body_part.part_type, PartType::Capsule { .. });
                let mut cone_value = matches!(body_part.part_type, PartType::Cone { .. });

                ui.selectable_value(&mut sphere_value, true, "⬤ Sphere");
                ui.selectable_value(&mut capsule_value, true,  "💊 Capsule");
                ui.selectable_value(&mut cone_value, true, "🔺Cone");
            });
            ui.end_row();

            ui.strong("");
            ui.horizontal(|ui| {
                ui.disable();
                let mut torus_value  = matches!(body_part.part_type, PartType::Torus { .. });
                let mut cylinder_value = matches!(body_part.part_type, PartType::Cylinder { .. });

                ui.selectable_value(&mut torus_value,  true, "🍩 Torus");
                ui.selectable_value(&mut cylinder_value, true, "🥫 Cylinder");
            });
            ui.end_row();
        });
        ui.separator();
    } 

    fn material_section(ui: &mut Ui, body_material: &mut BodyMaterial) {
        ui.heading("Material");
        ui.separator();

        egui::Grid::new("Material").spacing([8.0, 8.0]).show(ui, |ui| {
            ui.strong("Color");
            let linear = body_material.base_color.to_linear();
            let mut egui_color = egui::Color32::from_rgba_premultiplied(
                (linear.red * 255.0) as u8,
                (linear.green * 255.0) as u8,
                (linear.blue * 255.0) as u8,
                (linear.alpha * 255.0) as u8,
            );
            let color_picker = egui::color_picker::color_edit_button_srgba(
                ui,
                &mut egui_color,
                egui::color_picker::Alpha::Opaque,
            );

            if color_picker.changed() {
                let (r, g, b, a) = egui_color.to_tuple();
                body_material.base_color = Color::srgba(
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                    a as f32 / 255.0,
                );
            }
            ui.end_row();
            ui.strong("Roughness");
            ui.add(egui::Slider::new(&mut body_material.roughness, 0.0..=1.0));
            ui.end_row();

            ui.strong("Metallic");
            ui.add(egui::Slider::new(&mut body_material.metallic, 0.0..=1.0));
            ui.end_row();
        });
        ui.separator();
    }

    fn mode_section(ui: &mut Ui, local_mode: &mut GizmosMode, symmetry_mode: &mut SymmetryMode) {
        ui.heading("Mode");
        ui.separator();

        egui::Grid::new("Mode").spacing([8.0, 8.0]).show(ui, |ui| {
            ui.strong("Gizmos Mode");
            ui.horizontal(|ui| {
                ui.selectable_value(local_mode, GizmosMode::Translate, "Translate (T)");
                ui.selectable_value(local_mode, GizmosMode::Scale, "Scale (S)");
                ui.selectable_value(local_mode, GizmosMode::Rotate, "Rotate (R)");
            });
            
            ui.end_row();

            ui.strong("Symmetry Mode");
            ui.add(egui::Checkbox::new(
                &mut symmetry_mode.enabled,
                "Enable Symmetry Mode",
            ));
        });

        ui.separator();
    }

    fn hierarchy_section<'a>(
        ui: &mut Ui, 
        selected_entity: &Entity, 
        body_parts: &'a Query<'a, 'a, (Entity, &'static BodyPart)>, 
        children_of: &'a Query<'a, 'a, &'static ChildOf>,
        transform_ctx: &mut TransformContext, 
        commands: &mut Commands,
        history: &mut EditHistory
    ) {
        ui.heading("Hierarchy");
        ui.separator(); 

        let mut children_map: HashMap<Entity, Vec<Entity>> = HashMap::new();
        let mut roots: Vec<Entity> = Vec::new();

        for (entity, _) in body_parts.iter() {
            match children_of.get(entity) {
                Ok(child_of) => children_map.entry(child_of.parent()).or_default().push(entity),
                Err(_) => roots.push(entity),
            }
        }

        // Render drop body part zone
        let (_, payload) = ui.dnd_drop_zone::<Entity, _>(
            egui::Frame::default()
                .inner_margin(Margin::symmetric(16, 16))
                .corner_radius(5.0)
                .fill(egui::Color32::from_rgba_unmultiplied(80, 80, 80, 60)) 
                .stroke(egui::Stroke::new(2.0, egui::Color32::DARK_GRAY)),
            |ui| {
                ui.label("⬆ Drop body part here to detach from parent")
            }
        );
        
        let hierarchy_reference = HierarchyReference{
            children_map: &children_map,
            body_parts,
            children_of 
        };

        if let Some(dragged_entity) = payload {
            BodyHierarchy::assign_to_root(&dragged_entity, &hierarchy_reference, transform_ctx, commands, history); 
        }


        for entity in &roots {
            let entity_reference = EntityReference { selected_entity, entity };
            Self::hierarchy_node(ui, 0, &entity_reference, &hierarchy_reference, transform_ctx, commands, history);
        } 
        
        ui.separator();
    } 

    fn hierarchy_node(
        ui: &mut Ui,
        depth: usize,
        entity_reference: &EntityReference, 
        hierarchy_reference: &HierarchyReference,
        transform_ctx: &mut TransformContext,
        commands: &mut Commands,
        history: &mut EditHistory 
    ) {
        let entity = entity_reference.entity;
        let selected_entity = entity_reference.selected_entity;
        let Ok((_, body_part)) = hierarchy_reference.body_parts.get(*entity) else {return;};
        let is_entity_selected = selected_entity == entity;
        let indent = "      ".repeat(depth);
        let label = if is_entity_selected {format!("{} ▶ {}", indent, body_part.name)} 
            else {format!("  {} • {}", indent, body_part.name)};

        // drag source and use entity as the payload
        let item_response = ui.dnd_drag_source(
            egui::Id::new(("part_drag", body_part.id)), 
            *entity,
            |ui| {ui.label(&label)}
        ).response;

        if let Some(dragged_entity) = item_response.dnd_release_payload::<Entity>() {
            BodyHierarchy::assign_to_parent(&dragged_entity, entity_reference, hierarchy_reference, transform_ctx, commands, history);
        };

        if let Some(children) = hierarchy_reference.children_map.get(entity) {
            for entity in children {
                let entity_reference = EntityReference { selected_entity, entity };
                Self::hierarchy_node(ui, depth + 1, &entity_reference, hierarchy_reference, transform_ctx, commands, history);
            } 
        }; 
    } 

    fn shape_section(ui: &mut Ui, entity: Entity, body_part: &BodyPart, pending: &mut PendingResize, pending_sculpt_reset: &mut PendingSculptReset){
        ui.heading("Shape");
        ui.separator();

        if body_part.is_sculpted {
            ui.colored_label(egui::Color32::YELLOW, "⚠️ Reset Sculpt to Resize");
            let reset_button = ui.button("Reset Sculpt");

            if reset_button.clicked() {
                pending_sculpt_reset.entity = Some(entity);
            }

            ui.separator();
            return;
        } 
        let mut changed = false;

        egui::Grid::new("Shape").spacing([8.0, 8.0]).show(ui, |ui| {
            match body_part.part_type {
                PartType::Sphere { radius } => {
                    Self::radius_slider(ui, &radius, pending, &mut changed);  
                    ui.end_row();

                    Self::subdivisions_slider(ui, body_part, pending, &mut changed);
                    ui.end_row();
                },
                PartType::Capsule { radius, half_length } => {
                    Self::radius_slider(ui, &radius, pending, &mut changed);  
                    ui.end_row();
 
                    Self::half_length_slider(ui, &half_length, pending, &mut changed);
                    ui.end_row();

                    Self::subdivisions_slider(ui, body_part, pending, &mut changed);
                    ui.end_row();
                },
                PartType::Cone { radius, height} => {
                    Self::radius_slider(ui, &radius, pending, &mut changed);  
                    ui.end_row();

                    Self::height_slider(ui, &height, pending, &mut changed); 
                    ui.end_row();
                },
                PartType::Torus { major_radius, minor_radius } => {
                    Self::major_radius_slider(ui, &major_radius, pending, &mut changed); 
                    ui.end_row();

                    Self::minor_radius_slider(ui, &minor_radius, pending, &mut changed);        
                    ui.end_row();
                },
                PartType::Cylinder { radius, half_height } => {
                    Self::radius_slider(ui, &radius, pending, &mut changed); 
                    ui.end_row();

                    Self::half_height_slider(ui, &half_height, pending, &mut changed);        
                    ui.end_row();
                },
            } 
        });

        if changed {
            pending.entity = Some(entity);
        }
        ui.separator();
    }

    fn sculpt_section(ui: &mut Ui, brush: &mut SculptBrush, mode: &SculptMode) {
        if *mode != SculptMode::Sculpt {return;}; 
        ui.heading("Sculpt Brush");
        ui.separator();

        egui::Grid::new("SculptBrush").spacing([8.0, 8.0])
            .show(ui, |ui| {
                ui.strong("Strength");
                ui.add(egui::Slider::new(&mut brush.strength, 0.01..=1.0));
                ui.end_row();

                ui.strong("Radius");
                ui.add(egui::Slider::new(&mut brush.radius, 0.1..=3.0));
                ui.end_row();
            });
        ui.separator();
    }

    fn subdivisions_slider(ui: &mut Ui, body_part: &BodyPart, pending: &mut PendingResize, changed: &mut bool) {
        ui.strong("Subdivisions");
        ui.label("⚠ Resets sculpt");
        ui.end_row();

        let mut input_subdivisions = pending.subdivisions.unwrap_or(body_part.subdivisions);
        let subdivisions_slider = ui.add(egui::Slider::new(&mut input_subdivisions, 1u32..=5u32));
        
        if subdivisions_slider.changed() {
            pending.subdivisions = Some(input_subdivisions);
            *changed = true;
        }
    }

    fn radius_slider(ui: &mut Ui, default_radius: &f32, pending: &mut PendingResize, changed: &mut bool) {
        ui.strong("Radius");
        let mut input_radius = pending.radius.unwrap_or(*default_radius);
        let radius_field = ui.add(egui::Slider::new(&mut input_radius, 0.1..=2.0));
        if radius_field.changed() {
            pending.radius = Some(input_radius);
            *changed = true;
        }
    }

    fn height_slider(ui: &mut Ui, default_height: &f32, pending: &mut PendingResize, changed: &mut bool) {
        ui.strong("Height");
        let mut input_height = pending.height.unwrap_or(*default_height);
        let height_field = ui.add(egui::Slider::new(&mut input_height, 0.1..=5.0));
        if height_field.changed() {
            pending.height = Some(input_height);
            *changed = true;
        }
    }

    fn half_length_slider(ui: &mut Ui, default_half_length: &f32, pending: &mut PendingResize, changed: &mut bool) {
        ui.strong("Half-length");
        let mut input_half_length = pending.half_length.unwrap_or(*default_half_length);
        let half_length_field = ui.add(egui::Slider::new(&mut input_half_length, 0.1..=5.0));

        if half_length_field.changed() {
            pending.half_length = Some(input_half_length);
            *changed = true;
        }
    }

    fn half_height_slider(ui: &mut Ui, default_half_height: &f32, pending: &mut PendingResize, changed: &mut bool) {
        ui.strong("Half-Height");
        let mut input_half_height = pending.half_height.unwrap_or(*default_half_height);
        let half_height_field = ui.add(egui::Slider::new(&mut input_half_height, 0.1..=5.0));
        if half_height_field.changed() {
            pending.half_height = Some(input_half_height);
            *changed = true;
        }
    }

    fn major_radius_slider(ui: &mut Ui, default_major_radius: &f32, pending: &mut PendingResize, changed: &mut bool) {
        ui.strong("Major-Radius");
        let mut input_major_radius = pending.major_radius.unwrap_or(*default_major_radius);
        let major_radius_field = ui.add(egui::Slider::new(&mut input_major_radius, 0.1..=2.0));
        if major_radius_field.changed() {
            pending.major_radius = Some(input_major_radius);
            *changed = true;
        }
    }

    fn minor_radius_slider(ui: &mut Ui, default_minor_radius: &f32, pending: &mut PendingResize, changed: &mut bool) {
        ui.strong("Minor-Radius");
        let mut input_minor_radius = pending.minor_radius.unwrap_or(*default_minor_radius);
        let minor_input_field = ui.add(egui::Slider::new(&mut input_minor_radius, 0.1..=2.0));
        if minor_input_field.changed() {
            pending.minor_radius = Some(input_minor_radius);
            *changed = true;
        }
    }
}
