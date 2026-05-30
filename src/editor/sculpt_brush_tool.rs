use bevy::input::mouse::MouseWheel;
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use bevy_egui::EguiContexts;
use super::resource::{ControlContext, SceneContext, SculptBrush, SculptMode, SpawnContext, TransformContext};

pub struct SculptBrushTool;

impl SculptBrushTool {
    pub fn handle_brush(
        sculpt_mode: Res<SculptMode>,
        brush: Res<SculptBrush>,
        control_ctx: ControlContext,
        mut spawn_ctx: SpawnContext,
        transform_ctx: TransformContext,
        scene_ctx: SceneContext,
        mut egui_ctxs: EguiContexts
    ) {
        if *sculpt_mode != SculptMode::Sculpt {return;};
        if !control_ctx.buttons.pressed(MouseButton::Left) {return;};
        if egui_ctxs.ctx_mut().expect("egui context").wants_pointer_input() {return;};

        let Some((mesh, entity_world)) = Self::get_selected_mesh(&control_ctx, &mut spawn_ctx, &transform_ctx) else {return;};
        let Some(ray) = Self::get_cursor_ray(&scene_ctx) else {return;}; 
        let Some(vertices) = Self::get_vertices(mesh) else {return;};
        let Some(local_brush_contact_point) = Self::get_local_brush_contact_point(&entity_world, &ray) else {return;};
        
        // Applying brush on mesh vertices
        let brush_radius_squared = brush.radius * brush.radius; 
        
        for vertex in vertices.iter_mut() {
            let vertex_position = Vec3::from(*vertex);
            let distance_squared = vertex_position.distance_squared(local_brush_contact_point);
            let is_within_brush_effective_radius = distance_squared < brush_radius_squared;

            if is_within_brush_effective_radius {
                let falloff = 1.0 - (distance_squared / brush_radius_squared).sqrt();
                let delta = Vec3::Y * brush.strength * falloff;
                *vertex = (vertex_position + delta).to_array();
            }
        }

        mesh.compute_normals();
    }

    pub fn handle_brush_radius_change(
        mode: Res<SculptMode>,
        mut scroll: MessageReader<MouseWheel>,
        mut brush: ResMut<SculptBrush>
    ) {
        if *mode != SculptMode::Sculpt {return;};
        for event in scroll.read() {
            brush.radius = (brush.radius + event.y*0.05).clamp(0.05, 5.0);
        }
    }

    pub fn handle_brush_cursor(
        mode: Res<SculptMode>,
        brush: Res<SculptBrush>,
        scene_ctx: SceneContext,
        control_ctx: ControlContext,
        transform_ctx: TransformContext,
        mut gizmos: Gizmos
    ) {
        if *mode != SculptMode::Sculpt {return;};
        let Some(entity) = control_ctx.selection.entity else {return;};
        let Some(cursor_ray) = Self::get_cursor_ray(&scene_ctx) else {return;};
        let Ok(entity_world) = transform_ctx.global_transform_query.get(entity) else {return;};
        let entity_position = entity_world.translation();
        let Some(distance) = cursor_ray.intersect_plane(entity_position, InfinitePlane3d::new(-cursor_ray.direction)) else {return;};
        let brush_contact_point_world = cursor_ray.get_point(distance);

        let rotation = Quat::from_rotation_arc(Vec3::Z, -*cursor_ray.direction);
        gizmos.circle(Isometry3d::new(brush_contact_point_world, rotation), brush.radius, Color::WHITE);
    }

    fn get_selected_mesh<'a>(control_ctx: &ControlContext, spawn_ctx: &'a mut SpawnContext, transform_ctx: &TransformContext) -> Option<(&'a mut Mesh, GlobalTransform)> {
        let entity = control_ctx.selection.entity?;
        let mesh_handle = spawn_ctx.mesh3d_query.get(entity).ok()?.0.clone();
        let mesh = spawn_ctx.meshes.get_mut(&mesh_handle)?;
        let entity_world = transform_ctx.global_transform_query.get(entity).ok()?;

        Some((mesh, *entity_world)) 
    }

    fn get_cursor_ray(scene_ctx: &SceneContext) -> Option<Ray3d>{
        let window = scene_ctx.window_query.single().ok()?;
        let cursor_position = window.cursor_position()?;
        let (camera, camera_global_transform) = scene_ctx.camera_query.single().ok()?;
        let ray = camera.viewport_to_world(camera_global_transform, cursor_position).ok()?;

        ray.into()
    }

    fn get_vertices(mesh: &mut Mesh) -> Option<&mut Vec<[f32; 3]>>{
        let positions = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)?;
        if let VertexAttributeValues::Float32x3(vertices) = positions {
            Some(vertices)
        }else {
            None
        } 
    }

    fn get_local_brush_contact_point(entity_world: &GlobalTransform, ray: &Ray3d) -> Option<Vec3>{
        let entity_position = entity_world.translation();
        let distance = ray.intersect_plane(entity_position, InfinitePlane3d::new(-ray.direction))?;
        let brush_contact_point_world = ray.get_point(distance);
        let local_brush_contact_point = entity_world.affine().inverse().transform_point3(brush_contact_point_world);

        Some(local_brush_contact_point)
    }
}
