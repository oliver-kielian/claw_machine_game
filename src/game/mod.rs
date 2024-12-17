use bevy::prelude::*;

pub mod ball;
pub mod cat;
pub mod claw;
pub mod text;

use crate::game::ball::BallPlugin;
use crate::game::cat::CatPlugin;
use crate::game::claw::ClawPlugin;
use crate::game::text::TextPlugin;

///Plugin to allow for better orginization
pub struct GamePlugin;

///Impliments Plugin trait for TextPlugin
///Insets the relevant systems and resources to the Game logic of the app
impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(BallPlugin)
            .add_plugins(CatPlugin)
            .add_plugins(ClawPlugin)
            .add_plugins(TextPlugin);
    }
}