use bevy::picking::pointer::{PointerInteraction};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::model::body_part::BodyPart;
use crate::editor::gizmos::GizmosHandle;

use super::resource::SculptMode;

#[derive(Resource, Default)]
pub struct Selection {
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct OriginalMaterial(pub Handle<StandardMaterial>);

pub struct Selector;

impl Selector {
    pub fn on_press(mut event: On<Pointer<Press>>, mut selection: ResMut<Selection>) {
        event.propagate(false);
        selection.entity = Some(event.entity);
    }

    pub fn handle_deselect( 
        pointer_interaction_query: Query<&PointerInteraction>,
        body_part_query: Query<(), With<BodyPart>>,
        gizmos_handle_query: Query<(), With<GizmosHandle>>,
        buttons: Res<ButtonInput<MouseButton>>,
        mode: Res<SculptMode>,
        mut selection: ResMut<Selection>,
        mut egui_contexts: EguiContexts,
    ) {
        if *mode != SculptMode::Select {return;};
        if !buttons.just_pressed(MouseButton::Left) { return; }
        if egui_contexts.ctx_mut().expect("egui context").wants_pointer_input() { return; }

        // if pointer clicking body or gizmos handle entity
        let hit_any_body_parts: bool = pointer_interaction_query 
            .iter()
            .filter_map(|pointer| pointer.get_nearest_hit())
            .any(|(entity, _)| body_part_query.contains(*entity) || gizmos_handle_query.contains(*entity));

        if !hit_any_body_parts { selection.entity = None; }
    }

    pub fn handle_button_input(keys: Res<ButtonInput<KeyCode>>, mut selection: ResMut<Selection>) {
        if keys.just_pressed(KeyCode::Escape) { selection.entity = None; }
    }
}
