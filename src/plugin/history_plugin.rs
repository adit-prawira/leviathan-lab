use bevy::prelude::*;
use crate::history::edit_history::{EditHistory, EditHistoryManager};

pub struct HistoryPlugin;

impl Plugin for HistoryPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(EditHistory::default())
            .add_systems(Update, (
                EditHistoryManager::handle_undo, 
                EditHistoryManager::handle_redo, 
                EditHistoryManager::handle_record
            ).chain()); 
    }
}
