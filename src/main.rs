use bevy::prelude::*;
use core_input_plugin::prelude::InputCorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputCorePlugin)
        .run();
}
