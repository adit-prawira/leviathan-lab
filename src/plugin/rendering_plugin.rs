use bevy::prelude::*;

use crate::rendering::screen::Screen;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,
            Screen::spawn_lights
        );
    }
}
