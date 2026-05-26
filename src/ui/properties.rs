use crate::editor::gizmos::GizmosMode;
use crate::editor::sculpt_tool::PendingResize;
use crate::history::edit_history::{Action, EditHistory};
use crate::model::body_material::BodyMaterial;
use crate::model::body_part::{BodyPart, PartType};
use crate::editor::selector::Selection;
use crate::editor::symmetry::SymmetryMode;
use bevy::ecs::system::SystemParam;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_egui::egui::{Margin, Response, Ui};
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
    history: ResMut<'w, EditHistory>,
    pending_resize: ResMut<'w, PendingResize>
}

#[derive(SystemParam)]
pub struct SelectionContext<'w, 's>{
    selection: Res<'w, Selection>,
    body_parts: Query<'w, 's, (Entity, &'static BodyPart)>,
    children_of: Query<'w, 's, &'static ChildOf>
}

struct HierarchyReference<'a>{
    children_map: &'a HashMap<Entity, Vec<Entity>>,
    body_parts: &'a Query<'a, 'a, (Entity, &'static BodyPart)>,
    children_of: &'a Query<'a, 'a, &'static ChildOf>,
}

struct EntityReference<'a> {
    selected_entity: &'a Entity,
    entity: &'a Entity
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

                Self::render_shape_section(ui, entity, body_part, &mut editor_ctx.pending_resize); 

                Self::render_material_section(ui, &mut body_material);  

                let current_mode = *editor_ctx.mode;
                let mut local_mode = current_mode;
                
                Self::render_mode_section(ui, &mut local_mode, &mut editor_ctx.symmetry_mode);
                
                if local_mode != current_mode {*editor_ctx.mode = local_mode;}; 

                Self::render_hierarchy_section(
                    ui, 
                    &entity, 
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
        ui.separator();
    } 

    fn render_material_section(ui: &mut Ui, body_material: &mut BodyMaterial) {
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

        ui.separator();
    }

    fn render_hierarchy_section<'a>(
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
        
        if let Some(dragged_entity) = payload {
           Self::assign_to_root(&dragged_entity, children_of, transform_ctx, commands, history); 
        }

        let hierarchy_reference = HierarchyReference{
            children_map: &children_map,
            body_parts,
            children_of 
        };

        for entity in &roots {
            let entity_reference = EntityReference { selected_entity, entity };
            Self::render_hierarchy_node(ui, 0, &entity_reference, &hierarchy_reference, transform_ctx, commands, history);
        } 
        
        ui.separator();
    } 

    fn render_hierarchy_node(
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
            Self::assign_to_parent(&dragged_entity, entity_reference, hierarchy_reference, transform_ctx, commands, history);
        };

        if let Some(children) = hierarchy_reference.children_map.get(entity) {
            for entity in children {
                let entity_reference = EntityReference { selected_entity, entity };
                Self::render_hierarchy_node(ui, depth + 1, &entity_reference, hierarchy_reference, transform_ctx, commands, history);
            } 
        }; 
    }

    fn assign_to_root(
        dragged_entity: &Entity,
        children_of: &Query<&ChildOf>,
        transform_ctx: &mut TransformContext, 
        commands: &mut Commands,
        history: &mut EditHistory
    ) {
        let dragged = *dragged_entity;
        if let Some(old_parent) = children_of.get(dragged).ok().map(|children| children.parent()) {
            let old_transform = transform_ctx.transforms.get(dragged).copied().unwrap_or_default();
            let old_world = transform_ctx.global_transforms.get(dragged).copied().unwrap_or_default();
            let local_transform = Mat4::from(old_world.affine());
            let new_transform = Transform::from_matrix(local_transform);
            if let Ok(mut transform) = transform_ctx.transforms.get_mut(dragged) {
                *transform = new_transform;
            }
            commands.entity(dragged).remove::<ChildOf>();
            history.undo_stacks.push(Action::AssignParentEntity { 
                entity: dragged, 
                old_parent: Some(old_parent), 
                old_transform, 
                new_parent: None, 
                new_transform 
            });
            history.redo_stacks.clear();
        } 
    }

    fn assign_to_parent(
        dragged_entity: &Entity,
        entity_reference: &EntityReference, 
        hierarchy_reference: &HierarchyReference,
        transform_ctx: &mut TransformContext,
        commands: &mut Commands,
        history: &mut EditHistory         
    ) {
        let entity = entity_reference.entity; 
        let dragged = *dragged_entity;
        let should_change_hierarychy = dragged != *entity
            && !Self::is_descendant(*entity, dragged, hierarchy_reference.children_of);

        if should_change_hierarychy {
            let old_parent = hierarchy_reference.children_of.get(dragged).ok().map(|child_of| child_of.parent());
            let new_parent = Some(*entity);

            let old_world = transform_ctx.global_transforms.get(dragged).copied().unwrap_or_default();
            let parent_world = transform_ctx.global_transforms.get(*entity).copied().unwrap_or_default();
            let old_transform = transform_ctx.transforms.get(dragged).copied().unwrap_or_default();
            let local_parent_transform = Mat4::from(parent_world.affine().inverse() * old_world.affine());
            let new_transform = Transform::from_matrix(local_parent_transform);

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
        };
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

    fn render_shape_section(ui: &mut Ui, entity: Entity, body_part: &BodyPart, pending: &mut PendingResize){
        ui.heading("Shape");
        ui.separator();

        let mut changed = false;

        egui::Grid::new("Shape").spacing([8.0, 8.0]).show(ui, |ui| {
            match body_part.part_type {
                PartType::Sphere { radius } => {
                    ui.strong("Radius");
                    let mut input_radius = pending.radius.unwrap_or(radius);
                    
                    let radius_field = ui.add(egui::Slider::new(&mut input_radius, 0.1..= 2.0));
                    if radius_field.changed() {
                        pending.radius = Some(input_radius);
                        changed = true;
                    }
                },
                PartType::Capsule { radius, half_length } => {
                    ui.strong("Radius");
                    let mut input_radius = pending.radius.unwrap_or(radius);
                    let radius_field = ui.add(egui::Slider::new(&mut input_radius, 0.1..=2.0));
                    
                    if radius_field.changed() {
                        pending.radius = Some(input_radius);
                        changed = true;
                    }
                    ui.end_row();

                    ui.strong("Half-length");
                    let mut input_half_length = pending.half_length.unwrap_or(half_length);
                    let half_length_field = ui.add(egui::Slider::new(&mut input_half_length, 0.1..=5.0));

                    if half_length_field.changed() {
                        pending.half_length = Some(input_half_length);
                        changed = true;
                    }
                }
            }
        });

        if changed {
            pending.entity = Some(entity);
        }

        ui.separator();
    }
}
