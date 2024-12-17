use bevy::prelude::*;


///Controls whether the ball is attached to the claw or not
#[derive(Resource)]
pub struct BallState {
    pub is_attached: bool,
}
///Default implimentatoin is false
impl Default for BallState {
    fn default() -> Self {
        BallState { is_attached: false }
    }
}

///Controls when the game is won
#[derive(Resource)]
pub struct Game{
    pub win : bool
}

///Default implimentatoin is false
impl Default for Game {
    fn default() -> Self {
        Game { win: false}
    }
}