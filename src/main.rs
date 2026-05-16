use bevy::window::WindowResolution;
use bevy::{ prelude::*};
use leviathan_lab::{camera, screen};

fn main() {
    App::new().add_plugins(
        DefaultPlugins.build().disable::<GilrsPlugin>().set(WindowPlugin{
            primary_window: Some(Window {
                title: "Leviathan Lab".into(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GlobalAmbientLight{
            color: Color::srgb(0.05, 0.1, 0.02),
            brightness: 0.2,
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.01, 0.02, 0.05))) 
        .add_systems(Startup, (
            camera::Camera::spawn, 
            screen::Screen::spawn_lights, 
            screen::Screen::spawn_object
        ))
        .run();
}
