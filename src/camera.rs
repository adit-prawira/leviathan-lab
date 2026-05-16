use bevy::prelude::*;

pub struct Camera;

impl Camera {
    pub fn spawn(mut commands: Commands) {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(6.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y)
        ));
    }
}
