use crate::body_material::BodyMaterial;
use crate::model::body_part::BodyPart;
use crate::selector::Selection;
use crate::symmetry::SymmetryMode;
use bevy::prelude::*;
use bevy_egui::egui::Response;
use bevy_egui::{EguiContexts, egui};

pub struct PropertiesPanel;

/**
 * UI panel (Bevy UI or `bevy_egui`)
 * Shows selected part: name, position (x/y/z),
 *                      rotation (euler), scale (x/y/z)
 * Numeric fields editable (text input or drag-value)
 * Updates part transform in real time
 */
impl PropertiesPanel {
    pub fn show(
        selection: Res<Selection>,
        body_parts: Query<&BodyPart>,
        mut contexts: EguiContexts,
        mut symmetry_mode: ResMut<SymmetryMode>,
        mut transforms: Query<&mut Transform>,
        mut body_materials: Query<&mut BodyMaterial>,
    ) {
        let Some(entity) = selection.entity else { return; };
        let Ok(body_part) = body_parts.get(entity) else { return; };
        let Ok(mut transform) = transforms.get_mut(entity) else { return; };
        let Ok(mut body_material) = body_materials.get_mut(entity) else { return; };

        // render panel on the right-hand side of the screen
        egui::SidePanel::right("properties").min_width(400.0).show(
            contexts.ctx_mut().expect("egui context"),
            |ui| {
                ui.heading("Properties");
                ui.separator();

                egui::Grid::new("Info").spacing([8.0, 8.0]).show(ui, |ui| {
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

                ui.heading("Color");
                ui.add_space(8.0);
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
                ui.separator();

                ui.heading("Texture");
                egui::Grid::new("Texture")
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        ui.strong("Roughness");
                        ui.add(egui::Slider::new(&mut body_material.roughness, 0.0..=1.0));
                        ui.end_row();

                        ui.strong("Metallic");
                        ui.add(egui::Slider::new(&mut body_material.metallic, 0.0..=1.0));
                        ui.end_row();
                    });

                ui.heading("Mode");
                egui::Grid::new("Mode").spacing([8.0, 8.0]).show(ui, |ui| {
                    ui.strong("Symmetry Mode");
                    ui.add(egui::Checkbox::new(
                        &mut symmetry_mode.enabled,
                        "Enable Symmetry Mode",
                    ));
                })
            },
        );
    }
}
