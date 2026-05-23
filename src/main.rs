use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use leviathan_lab::editor;
use leviathan_lab::editor::sculpt_tool::SculptTool;
use leviathan_lab::model::body_material::BodyMaterial;
use leviathan_lab::history::edit_history::{self, EditHistoryPlugin};
use leviathan_lab::editor::gizmos::{GizmosManager, GizmosMode};
use leviathan_lab::rendering::screen;
use leviathan_lab::scene::camera;
use leviathan_lab::ui::properties::PropertiesPanel;
use leviathan_lab::scene::spawner::Spawner;
use leviathan_lab::editor::symmetry::{self, Symmetry};
use leviathan_lab::editor::selector::{self};
use leviathan_lab::ui::toolbar::Toolbar;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<GilrsPlugin>().set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Leviathan Lab".into(),
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
            EguiPlugin::default(),
        ))
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.05, 0.1, 0.02),
            brightness: 0.2,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.01, 0.02, 0.05)))
        .insert_resource(selector::Selection::default())
        .insert_resource(symmetry::SymmetryMode::default())
        .insert_resource(symmetry::PendingSymmetricChanges::default())
        .insert_resource(edit_history::EditHistory::default())
        .insert_resource(editor::sculpt_tool::SculptMode::default())
        .insert_resource(editor::sculpt_tool::BodyPartType::default())
        .insert_resource(editor::sculpt_tool::BodyPartId::default())
        .add_systems(Startup, (camera::Camera::spawn, screen::Screen::spawn_lights, Spawner::spawn_monster))
        .insert_resource(GizmosMode::default())
        .add_systems(
            Update,
            (
                camera::Camera::orbit,
                selector::Selector::deselect_on_click_away,
                selector::Selector::input_shortcuts,
                GizmosManager::draw,
                GizmosManager::sync_handles,
                GizmosManager::mode_keys,
                GizmosManager::update_handle_position,
                BodyMaterial::sync,
                (
                    // changes must run collected before
                    // changes can be applied
                    Symmetry::collect_changes,
                    Symmetry::apply,
                ).chain(),
                (
                    EditHistoryPlugin::undo, 
                    EditHistoryPlugin::redo, 
                    EditHistoryPlugin::record
                ).chain(),
                SculptTool::mode_keys, 
                SculptTool::on_add_body_part,
                SculptTool::on_delete_body_part
            ),
        )
        .add_systems(EguiPrimaryContextPass, PropertiesPanel::show)
        .add_systems(EguiPrimaryContextPass, Toolbar::show)
        .run();
}
