use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use leviathan_lab::model::body_material::BodyMaterial;
use leviathan_lab::history::edit_history::{self, EditHistoryPlugin};
use leviathan_lab::editor::gizmos::{GizmosManager, GizmosMode};
use leviathan_lab::rendering::screen;
use leviathan_lab::scene::camera;
use leviathan_lab::ui::properties::PropertiesPanel;
use leviathan_lab::scene::spawner::Spawner;
use leviathan_lab::editor::symmetry::{self, Symmetry};
use leviathan_lab::editor::selector::{self};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<GilrsPlugin>().set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Leviathan Lab".into(),
                    resolution: WindowResolution::new(1280, 720),
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
                ).chain()
            ),
        )
        .add_systems(EguiPrimaryContextPass, PropertiesPanel::show)
        .run();
}
