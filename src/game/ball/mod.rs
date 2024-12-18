use bevy::prelude::*;

pub mod componets;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

///Struct that allows for better orginization
pub struct BallPlugin;

///Implimentation of the Plugin trait for BallPlugin, this allows all relevant resources and systems to be added to the main app
impl Plugin for BallPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(BallState::default())
            .insert_resource(Game::default())
            .add_systems(Update, spawn_ball)
            .add_systems(Update, move_ball)
            .add_systems(Update, drop_ball);
    }
}