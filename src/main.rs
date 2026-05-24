use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use leviathan_lab::plugin::edit_history_plugin::EditHistoryPlugin;
use leviathan_lab::plugin::gizmos_plugin::GizmosPlugin;
use leviathan_lab::plugin::model_plugin::ModelPlugin;
use leviathan_lab::plugin::scene_plugin::ScenePlugin;
use leviathan_lab::plugin::sculpt_tool_plugin::SculptToolPlugin;
use leviathan_lab::plugin::selector_plugin::SelectorPlugin;
use leviathan_lab::plugin::symmetry_plugin::SymmetryPlugin;
use leviathan_lab::plugin::ui_plugin::UiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<GilrsPlugin>()
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Leviathan Lab".into(),
                    mode: WindowMode::BorderlessFullscreen(
                        MonitorSelection::Primary
                    ),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
            EguiPlugin::default(),
            ModelPlugin,
            ScenePlugin,
            SelectorPlugin,
            GizmosPlugin,
            SymmetryPlugin,
            SculptToolPlugin,
            EditHistoryPlugin,
            UiPlugin
        ))
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.05, 0.1, 0.02),
            brightness: 0.2,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.01, 0.02, 0.05)))   
        .run();
}
