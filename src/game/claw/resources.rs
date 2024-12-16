use bevy::prelude::*;

#[derive(Resource)]
pub struct ClawState {
    pub is_moving: bool,
    pub down: bool,
    pub up: bool,
}

impl Default for ClawState {
    fn default() -> Self {
        ClawState {
            is_moving: false,
            down: false,
            up: true,
        }
    }
}