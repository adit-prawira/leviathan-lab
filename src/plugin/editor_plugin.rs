use bevy::prelude::*;

use crate::editor::gizmos::{GizmosManager, GizmosMode};
use crate::editor::resource::{BodyPartId, BrushMode, PendingResize, SculptBodyPartType, SculptBrush, SculptMode};
use crate::editor::sculpt_brush_tool::SculptBrushTool;
use crate::editor::sculpt_tool::{SculptTool};
use crate::editor::selector::{Selection, Selector};
use crate::editor::symmetry::{PendingSymmetricChanges, Symmetry, SymmetryMode};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SculptMode::default())
            .insert_resource(SculptBodyPartType::default())
            .insert_resource(SculptBrush::default())
            .insert_resource(BrushMode::default())
            .insert_resource(BodyPartId::default())
            .insert_resource(Selection::default())
            .insert_resource(SymmetryMode::default())
            .insert_resource(PendingSymmetricChanges::default())
            .insert_resource(PendingResize::default())
            .insert_resource(GizmosMode::default())
            .add_systems(Update, (
                Selector::handle_deselect,
                Selector::handle_button_input,
                SculptTool::handle_button_input, 
                SculptTool::handle_add_body_part,
                SculptTool::handle_delete_body_part,
                SculptTool::handle_resize,
                SculptBrushTool::handle_brush,
                SculptBrushTool::handle_brush_radius_change, 
                SculptBrushTool::handle_brush_cursor,
                (
                    // changes must run collected before
                    // changes can be applied
                    Symmetry::handle_change,
                    Symmetry::handle_apply,
                ).chain(),
                GizmosManager::handle_draw,
                GizmosManager::handle_sync,
                GizmosManager::handle_button_input,
                GizmosManager::handle_transform_change,
            ));
    }
}
