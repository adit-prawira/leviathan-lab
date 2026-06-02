use bevy::input::mouse::MouseWheel;
use bevy::math::FloatPow;
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use bevy_egui::EguiContexts;
use crate::editor::resource::BrushMode;

use super::resource::{ControlContext, SceneContext, SculptBrush, SculptMode, SpawnContext, TransformContext};

struct BrushInput<'a>{
    vertices: &'a mut Vec<[f32; 3]>,
    contact: Vec3,
    brush_radius: f32,
    strength: f32,
    ray: &'a Ray3d
}
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
        let mut input = BrushInput {
            vertices,
            contact: local_brush_contact_point,
            brush_radius: brush.radius,
            strength: brush.strength, 
            ray: &ray
        };
        match brush.mode {
            BrushMode::Pull => Self::apply_pull(&mut input),
            BrushMode::Push => Self::apply_push(&mut input),
            BrushMode::Smooth => Self::apply_smooth(&mut input),
            BrushMode::Flatten => Self::apply_flatten(&mut input),
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

    fn apply_pull(brush_input: &mut BrushInput) {
        let brush_radius_squared = brush_input.brush_radius.squared();
        let direction = -*brush_input.ray.direction;
        for vertex in brush_input.vertices.iter_mut() {
            let position = Vec3::from(*vertex);
            let distance_squared = position.distance_squared(brush_input.contact);
            let is_within_brush_radius = distance_squared < brush_radius_squared;
            
            if !is_within_brush_radius {continue;};

            let falloff = 1.0 - (distance_squared / brush_radius_squared).sqrt();
            let delta = direction*brush_input.strength*falloff;
            *vertex = (position + delta).to_array();
        }
    }

    fn apply_push(brush_input: &mut BrushInput) {
        let brush_radius_squared = brush_input.brush_radius.squared();
        let direction = *brush_input.ray.direction;
        for vertex in brush_input.vertices.iter_mut() {
            let position = Vec3::from(*vertex);
            let distance_squared = position.distance_squared(brush_input.contact);
            let is_within_brush_radius = distance_squared < brush_radius_squared;
            
            if !is_within_brush_radius {continue;};
            
            let falloff = 1.0 - (distance_squared / brush_radius_squared).sqrt();
            let delta = direction*brush_input.strength*falloff;
            *vertex = (position + delta).to_array();
        }
    }
    
    fn apply_smooth(brush_input: &mut BrushInput) {
        let mut sum = Vec3::ZERO;
        let mut count = 0;
        let brush_radius_squared = brush_input.brush_radius.squared();
        
        for vertex in brush_input.vertices.iter() {
            let position = Vec3::from(*vertex);
            let distance_squared = position.distance_squared(brush_input.contact);
            let is_within_brush_radius = distance_squared < brush_radius_squared;
            
            if !is_within_brush_radius {continue;};

            sum += position; 
            count += 1;
        }

        if count == 0 {return;};
        let average = sum/(count as f32);

        for vertex in brush_input.vertices.iter_mut() {
            let position = Vec3::from(*vertex);
            let distance_squared = position.distance_squared(brush_input.contact);
            let is_within_brush_radius = distance_squared < brush_radius_squared;

            if !is_within_brush_radius {continue;};

            let falloff = 1.0 - (distance_squared/brush_radius_squared).sqrt();
            *vertex = position.lerp(average, brush_input.strength*falloff).to_array();
        }
    }
    
    fn apply_flatten(brush_input: &mut BrushInput) {
        let brush_radius_squared = brush_input.brush_radius.squared();

        let mut sum_y = 0.0;
        let mut count = 0;

        for vertex in brush_input.vertices.iter() {
            let position = Vec3::from(*vertex);
            let distance_squared = position.distance_squared(brush_input.contact);
            let is_within_brush_radius = distance_squared < brush_radius_squared;
            if !is_within_brush_radius {continue;};

            sum_y += position.y;
            count += 1;
        }

        if count == 0 {return;}
        let average_y = sum_y / (count as f32);

        for vertex in brush_input.vertices.iter_mut() {
            let position = Vec3::from(*vertex);
            let distance_squared = position.distance_squared(brush_input.contact);
            let is_within_brush_radius = distance_squared < brush_radius_squared;
            if !is_within_brush_radius {continue;};

            let falloff = 1.0 - (distance_squared / brush_radius_squared).sqrt();
            let target = Vec3::new(position.x, average_y, position.z);
            *vertex = position.lerp(target, brush_input.strength*falloff).to_array();
        }
    } 
}
