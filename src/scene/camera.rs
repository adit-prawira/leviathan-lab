use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

pub struct Camera;

const MIN_DISTANCE: f32 = 1.0;
const MAX_DISTANCE: f32 = 20.0;
const ORBIT_SENSITIVITY: f32 = 0.005;
const PAN_SENSITIVITY: f32 = 0.005;
const ZOOM_SENSITIVITY: f32 = 0.05;

#[derive(Resource)]
pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub target: Vec3,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            yaw: -2.356,
            pitch: 0.644,
            distance: 8.25,
            target: Vec3::new(0.0, 0.0, 0.2),
        }
    }
}

impl Camera {
    pub fn spawn(mut commands: Commands) {
        let orbit = OrbitCamera::default();
        let transform = orbit_transform(&orbit);
        commands.insert_resource(orbit);
        commands.spawn((
            Camera3d::default(),
            DistanceFog {
                color: Color::srgba(0.02, 0.05, 0.1, 1.0),
                falloff: FogFalloff::Exponential { density: 0.25 },
                ..default()
            },
            transform,
        ));
    }

    pub fn handle_orbit(
        mut orbit: ResMut<OrbitCamera>,
        mut query: Query<&mut Transform, With<Camera3d>>,
        mut motion: MessageReader<MouseMotion>,
        mut scroll: MessageReader<MouseWheel>,
        buttons: Res<ButtonInput<MouseButton>>,
    ) {
        let mut delta: Vec2 = Vec2::ZERO;
        for event in motion.read() {
            delta += event.delta;
        }

        // Spin
        if buttons.pressed(MouseButton::Right) {
            orbit.yaw -= delta.x * ORBIT_SENSITIVITY;
            orbit.pitch -= delta.y * ORBIT_SENSITIVITY;
            orbit.pitch = orbit.pitch.clamp(-1.5, 1.5);
        }

        // Drag
        let distance = orbit.distance;
        if buttons.pressed(MouseButton::Middle) {
            let right: Vec3 = Vec3::new(orbit.yaw.cos(), 0.0, -orbit.yaw.sin());
            let up: Vec3 = Vec3::Y;
            orbit.target -= right * delta.x * PAN_SENSITIVITY * distance;
            orbit.target += up * delta.y * PAN_SENSITIVITY * distance;
        }

        // Zoom
        for event in scroll.read() {
            orbit.distance -= event.y * ZOOM_SENSITIVITY;
            orbit.distance = orbit.distance.clamp(MIN_DISTANCE, MAX_DISTANCE);
        }

        if let Ok(mut transform) = query.single_mut() {
            *transform = orbit_transform(&orbit);
        }
    }
}

fn orbit_transform(orbit: &OrbitCamera) -> Transform {
    let x: f32 = orbit.distance * orbit.pitch.cos() * orbit.yaw.sin();
    let y: f32 = orbit.distance * orbit.pitch.sin();
    let z: f32 = orbit.distance * orbit.pitch.cos() * orbit.yaw.cos();
    let eye: Vec3 = orbit.target + Vec3::new(x, y, z);
    Transform::from_translation(eye).looking_at(orbit.target, Vec3::Y)
}
