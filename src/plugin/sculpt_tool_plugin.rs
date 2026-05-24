use bevy::app::{Plugin, Update};
use crate::editor::sculpt_tool::{BodyPartId, BodyPartType, SculptMode, SculptTool};

pub struct SculptToolPlugin;

impl Plugin for SculptToolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(SculptMode::default())
        .insert_resource(BodyPartType::default())
        .insert_resource(BodyPartId::default())
        .add_systems(Update, (
            SculptTool::handle_button_input, 
            SculptTool::handle_add_body_part,
            SculptTool::handle_delete_body_part
        ));
    }
}
