use bevy::prelude::Plugin;
use ezinput::prelude::EZInputPlugin;

use crate::bindings::BirdhouseBinding;

pub struct InputCorePlugin;

impl Plugin for InputCorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EZInputPlugin::<BirdhouseBinding>::default());
    }
}