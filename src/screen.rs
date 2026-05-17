use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;
pub struct Screen;

impl Screen {
    pub fn spawn_lights(
        mut commands:Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>
    ) {
        commands.spawn((
            DirectionalLight {
                illuminance: 15_000.0,
                shadows_enabled: true,
                ..default() 
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -FRAC_PI_4, FRAC_PI_4, 0.0))
        ));

        let floor:Handle<Mesh> = meshes.add(Plane3d::default().mesh().size(10.0, 10.0).build());
        let floor_material: Handle<StandardMaterial> = materials.add(StandardMaterial{
            base_color: Color::srgb(0.03, 0.06, 0.1),
            perceptual_roughness: 0.9,
            ..default()
        });
        commands.spawn((
            Mesh3d(floor),
            MeshMaterial3d(floor_material),
            Transform::from_xyz(0.0, -1.5, 0.0)
        ));
    }
}
