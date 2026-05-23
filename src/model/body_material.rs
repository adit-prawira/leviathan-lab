use bevy::prelude::*;

#[derive(Component, Clone, PartialEq)]
pub struct BodyMaterial {
    pub base_color: Color,
    pub roughness: f32,
    pub metallic: f32,
}

/*
 * It ontains base color, roughness, and metallic values of materials
 * It will sync any body materials changes into bevy's StandardMaterial mesh
 * */
impl BodyMaterial {
    pub fn handle_sync(query: Query<(&BodyMaterial, &MeshMaterial3d<StandardMaterial>), Changed<BodyMaterial>>, mut materials: ResMut<Assets<StandardMaterial>>) {
        for (body_material, material_handle) in query {
            let Some(material) = materials.get_mut(material_handle) else {continue;};
            material.base_color = body_material.base_color;
            material.perceptual_roughness = body_material.roughness;
            material.metallic = body_material.metallic;
        }
    }
}
