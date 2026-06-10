use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use leviathan_lab::plugin::editor_plugin::EditorPlugin;
use leviathan_lab::plugin::history_plugin::HistoryPlugin;
use leviathan_lab::plugin::model_plugin::ModelPlugin;
use leviathan_lab::plugin::rendering_plugin::RenderingPlugin;
use leviathan_lab::plugin::scene_plugin::ScenePlugin;
use leviathan_lab::plugin::ui_plugin::UiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<GilrsPlugin>()
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Leviathan Lab".into(),
                    mode: WindowMode::BorderlessFullscreen(
                        MonitorSelection::Current
                    ),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
            EguiPlugin::default(),
            ModelPlugin,
            RenderingPlugin,
            ScenePlugin,
            EditorPlugin, 
            HistoryPlugin,
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
