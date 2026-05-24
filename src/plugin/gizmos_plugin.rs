use bevy::app::{Plugin, Update};
use crate::editor::gizmos::{GizmosManager, GizmosMode};

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GizmosMode::default())
            .add_systems(Update, (
                GizmosManager::handle_draw,
                GizmosManager::handle_sync,
                GizmosManager::handle_button_input,
                GizmosManager::handle_transform_change,
            ));
    }
}
