use bevy::prelude::*;

mod componets;
pub mod resources;
mod systems;

use resources::*;
use systems::*;
pub struct BallPlugin;

impl Plugin for BallPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(BallState::default())
            .insert_resource(Game::default())
            .add_systems(Update, spawn_ball)
            .add_systems(Update, move_ball)
            .add_systems(Update, drop_ball);
    }
}