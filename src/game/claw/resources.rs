use bevy::prelude::*;

///Resoruce that controls the state of the claw. 
#[derive(Resource)]
pub struct ClawState {
    pub is_moving: bool,
    pub down: bool,
    pub up: bool,
}

///Default implimentaion for ClawState Up is true, everything else is false. 
impl Default for ClawState {
    fn default() -> Self {
        ClawState {
            is_moving: false,
            down: false,
            up: true,
        }
    }
}