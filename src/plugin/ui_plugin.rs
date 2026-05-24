use bevy::app::{Plugin};
use bevy_egui::EguiPrimaryContextPass;

use crate::ui::properties::PropertiesPanel;
use crate::ui::toolbar::Toolbar;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(EguiPrimaryContextPass, PropertiesPanel::show)
        .add_systems(EguiPrimaryContextPass, Toolbar::show);
    }
}
