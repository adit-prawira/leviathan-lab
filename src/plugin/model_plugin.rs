use bevy::app::{Update, Plugin};
use crate::model::body_hierarchy::BodyHierarchy;
use crate::model::body_material::BodyMaterial;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(BodyHierarchy::default())
            .add_systems(Update,( 
                BodyMaterial::handle_sync, 
            ));
    }
}
