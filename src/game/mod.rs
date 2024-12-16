use bevy::prelude::*;

pub mod ball;
pub mod cat;
pub mod claw;
pub mod text;

use crate::game::ball::BallPlugin;
use crate::game::cat::CatPlugin;
use crate::game::claw::ClawPlugin;
use crate::game::text::TextPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(BallPlugin)
            .add_plugins(CatPlugin)
            .add_plugins(ClawPlugin)
            .add_plugins(TextPlugin);
    }
}