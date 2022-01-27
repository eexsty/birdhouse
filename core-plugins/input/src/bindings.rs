use ezinput::prelude::*;
use ezinput_macros::*;
use strum_macros::Display;

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
pub enum BirdhouseBinding {
    Movement(BirdhouseMovementBinding)
}

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
pub enum BirdhouseMovementBinding {
    Jump,
    Down,
    Left,
    Right
}
