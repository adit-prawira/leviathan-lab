use bevy::picking::pointer::PointerInteraction;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::model::body_part::BodyPart;
use crate::editor::gizmos::GizmosHandle;

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

    pub fn deselect_on_click_away(
        mut selection: ResMut<Selection>,
        pointers: Query<&PointerInteraction>,
        body_parts: Query<(), With<BodyPart>>,
        gizmos_handles: Query<(), With<GizmosHandle>>,
        buttons: Res<ButtonInput<MouseButton>>,
        mut egui_contexts: EguiContexts,
    ) {
        if !buttons.just_pressed(MouseButton::Left) { return; }
        if egui_contexts.ctx_mut().expect("egui context").wants_pointer_input() { return; }

        // if pointer clicking body or gizmos handle entity
        let hit_any_body_parts: bool = pointers
            .iter()
            .filter_map(|pointer| pointer.get_nearest_hit())
            .any(|(entity, _)| body_parts.contains(*entity) || gizmos_handles.contains(*entity));

        if !hit_any_body_parts { selection.entity = None; }
    }

    pub fn input_shortcuts(keys: Res<ButtonInput<KeyCode>>, mut selection: ResMut<Selection>) {
        if keys.just_pressed(KeyCode::Escape) { selection.entity = None; }
        // Placeholder, the intention is to delete an entity
        if keys.just_pressed(KeyCode::Delete) || keys.just_pressed(KeyCode::Backspace) { selection.entity = None; }
    }
}
