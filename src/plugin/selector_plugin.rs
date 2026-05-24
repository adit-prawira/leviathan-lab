use bevy::app::{Plugin, Update};
use crate::editor::selector::{Selection, Selector};

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Selection::default())
            .add_systems(Update, (
                Selector::handle_deselect,
                Selector::handle_button_input,
            ));
    }
}
