use bevy::prelude::*;
use crate::editor::symmetry::{PendingSymmetricChanges, Symmetry, SymmetryMode};

pub struct SymmetryPlugin;

impl Plugin for SymmetryPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app.insert_resource(SymmetryMode::default())
        .insert_resource(PendingSymmetricChanges::default())
        .add_systems(Update, (
            // changes must run collected before
            // changes can be applied
            Symmetry::handle_change,
            Symmetry::handle_apply,
        ).chain());
    }
}
