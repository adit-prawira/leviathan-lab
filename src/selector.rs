use bevy::prelude::*;
use bevy::picking::pointer::PointerInteraction;

#[derive(Resource, Default)]
pub struct Selection {
    pub entity: Option<Entity>
}

#[derive(Component)]
pub struct OriginalMaterial(pub Handle<StandardMaterial>);

pub struct Selector;

impl Selector {
    pub fn on_press(
        event: On<Pointer<Press>>,
        mut selection: ResMut<Selection>,
        body_parts: Query<&crate::model::body_part::BodyPart>
    ) {
        selection.entity = Some(event.entity);
        if let Ok(body_part) = body_parts.get(event.entity) {
            println!("Body Part Selected => {}", body_part.name)
        }
    }

    pub fn deselect_on_click_away(
        mut selection: ResMut<Selection>,
        pointers: Query<&PointerInteraction>,
        body_parts: Query<(), With<crate::model::body_part::BodyPart>>,
        buttons: Res<ButtonInput<MouseButton>> 
    ){
        if !buttons.just_pressed(MouseButton::Left) {return;}

        let hit_any_body_parts = pointers.iter()
            .filter_map(|pointer| pointer.get_nearest_hit())
            .any(|(entity, _)| body_parts.contains(*entity));

        if !hit_any_body_parts {
            selection.entity = None;
        }
    }
}
