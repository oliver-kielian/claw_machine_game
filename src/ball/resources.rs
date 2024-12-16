use bevy::prelude::*;

#[derive(Resource)]
pub struct BallState {
    pub is_attached: bool,
}

impl Default for BallState {
    fn default() -> Self {
        BallState { is_attached: false }
    }
}

#[derive(Resource)]
pub struct Game{
    pub win : bool
}

impl Default for Game {
    fn default() -> Self {
        Game { win: false}
    }
}