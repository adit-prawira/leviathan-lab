#[derive(Clone, Debug)]
pub struct MaterialData {
    pub base_color: [f32; 4],
    pub roughness: f32,
    pub metallic: f32
}

impl Default for MaterialData {
    fn default() -> Self {
        Self {
            base_color: [0.8, 0.8, 0.8, 1.0],
            roughness: 0.5, 
            metallic: 0.0
        }
    }
}
