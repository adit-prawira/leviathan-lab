use crate::editor::gizmos::GizmosMode;
use crate::history::edit_history::{Action, EditHistory};
use crate::model::body_material::BodyMaterial;
use crate::model::body_part::{BodyPart};
use crate::editor::selector::Selection;
use crate::editor::symmetry::SymmetryMode;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::egui::{Response, Ui};
use bevy_egui::{EguiContexts, egui};

pub struct PropertiesPanel;

const MIN_SIDE_PANEL_WIDTH: f32 = 150.0;

#[derive(SystemParam)]
pub struct TransformContext<'w, 's> {
    global_transforms: Query<'w, 's, &'static GlobalTransform>,
    transforms: Query<'w, 's, &'static mut Transform>
}

#[derive(SystemParam)]
pub struct EditorContext<'w> {
    mode: ResMut<'w, GizmosMode>,
    symmetry_mode: ResMut<'w, SymmetryMode>,
    history: ResMut<'w, EditHistory>
}

#[derive(SystemParam)]
pub struct SelectionContext<'w, 's>{
    selection: Res<'w, Selection>,
    body_parts: Query<'w, 's, (Entity, &'static BodyPart)>,
    children_of: Query<'w, 's, &'static ChildOf>
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
        mut body_materials: Query<&mut BodyMaterial>,
        mut commands: Commands,
        mut transform_ctx: TransformContext,
        mut editor_ctx: EditorContext
    ) {
        let Some(entity) = selection_ctx.selection.entity else { return; };
        let Ok((_, body_part)) = selection_ctx.body_parts.get(entity) else { return; }; 
        let Ok(mut body_material) = body_materials.get_mut(entity) else { return; };

        // render panel on the right-hand side of the screen
        egui::SidePanel::right("properties").min_width(MIN_SIDE_PANEL_WIDTH)
            .show(contexts.ctx_mut().expect("egui context to be available"),|ui| {
                Self::render_properties_section(ui, entity, body_part, &mut transform_ctx.transforms);
                ui.separator();
 
                Self::render_material_section(ui, &mut body_material); 
                ui.separator();

                let current_mode = *editor_ctx.mode;
                let mut local_mode = current_mode;
                Self::render_mode_section(ui, &mut local_mode, &mut editor_ctx.symmetry_mode);
                if local_mode != current_mode {*editor_ctx.mode = local_mode;};
                ui.separator();

                Self::render_hierarchy_section(
                    ui, &entity, 
                    &selection_ctx.body_parts, 
                    &selection_ctx.children_of,
                    &mut transform_ctx,
                    &mut commands, 
                    &mut editor_ctx.history 
                );
            },
        );
    }

    fn render_properties_section(ui: &mut Ui, entity: Entity, body_part: &BodyPart, transforms: &mut Query<&mut Transform>) {
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
        });
    } 

    fn render_material_section(ui: &mut Ui, body_material: &mut BodyMaterial) {
        ui.heading("Material");
        ui.separator();

        egui::Grid::new("Material")
            .spacing([8.0, 8.0])
            .show(ui, |ui| {
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
    }

    fn render_mode_section(ui: &mut Ui, local_mode: &mut GizmosMode, symmetry_mode: &mut SymmetryMode) {
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
    }

    fn render_hierarchy_section(
        ui: &mut Ui, 
        current_entity: &Entity, 
        body_parts: &Query<(Entity, &BodyPart)>, 
        children_of: &Query<&ChildOf>,
        transform_ctx: &mut TransformContext, 
        commands: &mut Commands,
        history: &mut EditHistory
    ) {
        ui.heading("Hierarchy");
        ui.separator();
        
        let body_part_list: Vec<(Entity, String, u32)> = body_parts.iter()
            .map(|(entity, body_part)| (entity, body_part.name.clone(), body_part.id))
            .collect();

        for (entity, name, id) in &body_part_list {
            let is_entity_selected = current_entity == entity;
            let label = if is_entity_selected {format!("▶ {}", name)} else {format!("  {}", name)};

            // drag source and use entity as the payload
            let item_response = ui.dnd_drag_source(
                egui::Id::new(("part_drag", id)), 
                *entity,
                |ui| {ui.label(&label)}
            ).response;

            let Some(dragged_entity) = item_response.dnd_release_payload::<Entity>() else {continue;};
            let dragged = *dragged_entity;
            let should_change_hierarychy = dragged != *entity
                && !Self::is_descendant(*entity, dragged, children_of);
            if !should_change_hierarychy {continue;};
            let old_parent = children_of.get(dragged).ok().map(|child_of| child_of.parent());
            let new_parent = Some(*entity);

            let old_world = transform_ctx.global_transforms.get(dragged).copied().unwrap_or_default();
            let parent_world = transform_ctx.global_transforms.get(*entity).copied().unwrap_or_default();
            let old_transform = transform_ctx.transforms.get(dragged).copied().unwrap_or_default();
            let new_transform = Transform::from_matrix(
                Mat4::from(parent_world.affine().inverse() * old_world.affine())
            );

            if let Ok(mut transform) = transform_ctx.transforms.get_mut(dragged) {
                *transform = new_transform;
            }

            commands.entity(*entity).add_child(dragged);

            history.undo_stacks.push(Action::AssignParentEntity { 
                entity: dragged, 
                old_parent, 
                old_transform, 
                new_parent, 
                new_transform 
            });
            history.redo_stacks.clear();
        }
        ui.separator();
    }

    fn is_descendant(candidate: Entity, of: Entity, children_of: &Query<&ChildOf>) -> bool {
        let mut current = candidate;
        loop {
            match children_of.get(current) {
                Ok(child_of) if child_of.parent() == of => return true,
                Ok(child_of) => current = child_of.parent(),
                Err(_) => return false
            }
        };
    }
}
