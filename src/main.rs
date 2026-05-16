use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins.build().disable::<GilrsPlugin>()).run();
}
