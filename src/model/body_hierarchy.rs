use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct BodyHierarchy {
    pub entities: HashMap<u32, Entity>,
}
