use bevy::prelude::*;

pub struct Camera;

impl Camera {
    pub fn spawn(mut commands: Commands) {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(-4.0, 6.0, -4.0).looking_at(Vec3::new(0.0, 0.0, 0.2), Vec3::Y)
        ));
    }
}
