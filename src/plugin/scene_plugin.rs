use bevy::app::{Plugin, Startup, Update};
use crate::rendering::screen::Screen;
use crate::scene::camera::Camera;
use crate::scene::spawner::Spawner;

pub struct ScenePlugin;

impl Plugin for ScenePlugin{
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, (
                Camera::spawn, 
                Screen::spawn_lights, 
                Spawner::spawn_monster)
            ).add_systems(Update, (
                Camera::handle_orbit,
            ));
    }
}
