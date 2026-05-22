use crate::scene::camera::OrbitCamera;
use crate::editor::selector::Selection;
use bevy::prelude::*;
use core::fmt;
use std::f32::consts::FRAC_PI_2;

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub enum GizmosMode {
    #[default]
    Translate,
    Rotate,
    Scale,
}

impl fmt::Display for GizmosMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GizmosMode::Scale => write!(f, "Scale"),
            GizmosMode::Rotate => write!(f, "Rotate"),
            GizmosMode::Translate => write!(f, "Translate")
        }
    }
}

#[derive(Component)]
pub struct GizmosHandle {
    pub axis: Vec3,
    pub target: Entity,
}

pub struct GizmosManager;

/**
 * Gizmo handles: translate (axis arrows), rotate (axis rings), scale (axis boxes)
 * Drag a handle to modify the part's transform in real time
 * Gizmo mode togglable (T = translate, R = rotate, S = scale)
 * Gizmo visual: colored by axis (R=X, G=Y, B=Z)
 */
impl GizmosManager {
    const LENGTH: f32 = 1.2;
    pub fn draw(
        selection: Res<Selection>,
        mode: Res<GizmosMode>,
        transforms: Query<&GlobalTransform>,
        mut gizmos: Gizmos,
    ) {
        let Some(entity) = selection.entity else { return; };
        let Ok(global_transform) = transforms.get(entity) else { return; };

        let position: Vec3 = global_transform.translation();

        match *mode {
            GizmosMode::Translate => {
                gizmos.arrow(
                    position,
                    position + Vec3::X * Self::LENGTH,
                    Color::srgb(1.0, 0.2, 0.2),
                );
                gizmos.arrow(
                    position,
                    position + Vec3::Y * Self::LENGTH,
                    Color::srgb(0.2, 1.0, 0.2),
                );
                gizmos.arrow(
                    position,
                    position + Vec3::Z * Self::LENGTH,
                    Color::srgb(0.2, 0.2, 1.0),
                );
            }
            GizmosMode::Rotate => {
                gizmos.circle(
                    Isometry3d::new(position, Quat::from_rotation_z(0.0)),
                    Self::LENGTH,
                    Color::srgb(1.0, 0.2, 0.2),
                );
                gizmos.circle(
                    Isometry3d::new(position, Quat::from_rotation_x(FRAC_PI_2)),
                    Self::LENGTH,
                    Color::srgb(0.2, 1.0, 0.2),
                );
                gizmos.circle(
                    Isometry3d::new(position, Quat::from_rotation_y(FRAC_PI_2)),
                    Self::LENGTH,
                    Color::srgb(0.2, 0.2, 1.0),
                );
            }
            GizmosMode::Scale => {
                gizmos.arrow(
                    position,
                    position + Vec3::X * Self::LENGTH,
                    Color::srgb(1.0, 0.2, 0.2),
                );
                gizmos.arrow(
                    position,
                    position + Vec3::Y * Self::LENGTH,
                    Color::srgb(0.2, 1.0, 0.2),
                );
                gizmos.arrow(
                    position,
                    position + Vec3::Z * Self::LENGTH,
                    Color::srgb(0.2, 0.2, 1.0),
                );
            }
        }
    }

    pub fn sync_handles(
        selection: Res<Selection>,
        mode: Res<GizmosMode>,
        handles: Query<Entity, With<GizmosHandle>>,
        transforms: Query<&GlobalTransform>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        if !selection.is_changed() && !mode.is_changed() { return; }

        // Despawn old handles
        for entity in &handles {
            commands.entity(entity).despawn();
        }

        let Some(target_entity) = selection.entity else { return; };
        let Ok(global_transform) = transforms.get(target_entity) else { return; };
        let position: Vec3 = global_transform.translation();
        let axes: [(Vec3, Color); 3] = [
            (Vec3::X, Color::srgb(1.0, 0.2, 0.2)),
            (Vec3::Y, Color::srgb(0.2, 1.0, 0.2)),
            (Vec3::Z, Color::srgb(0.2, 0.2, 1.0)),
        ];

        for (axis, color) in axes {
            let handle_position: Vec3 = position + axis * Self::LENGTH;
            commands
                .spawn((
                    Mesh3d(meshes.add(Sphere::new(0.08).mesh().build())),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: color,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(handle_position),
                    GizmosHandle {
                        axis,
                        target: target_entity,
                    },
                ))
                .observe(GizmosManager::on_drag);
        }
    }

    pub fn on_drag(
        drag: On<Pointer<Drag>>,
        handles: Query<&GizmosHandle>,
        orbit: Res<OrbitCamera>,
        mut transforms: Query<&mut Transform>,
    ) {
        let Ok(handle) = handles.get(drag.entity) else { return; };
        let Ok(mut transform) = transforms.get_mut(handle.target) else { return; };
        let scale: f32 = orbit.distance * 0.01;
        let delta = drag.delta;

        match handle.axis {
            axis if axis == Vec3::X => transform.translation.x += delta.x * scale,
            axis if axis == Vec3::Y => transform.translation.y -= delta.y * scale,
            axis if axis == Vec3::Z => transform.translation.z += delta.x * scale,
            _ => {}
        }
    }

    pub fn mode_keys(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<GizmosMode>) {
        if keys.just_pressed(KeyCode::KeyT) { *mode = GizmosMode::Translate; }
        if keys.just_pressed(KeyCode::KeyS) { *mode = GizmosMode::Scale; }
        if keys.just_pressed(KeyCode::KeyR) { *mode = GizmosMode::Rotate; }
    }
}
