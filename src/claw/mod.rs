use bevy::prelude::*;

pub mod componets;
mod systems;
pub mod resources;

use systems::*;
use resources::*;
pub struct ClawPlugin;

impl Plugin for ClawPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(ClawState::default())
        .add_systems(Update, move_claw)
        .add_systems(Update, claw_collisions)
        .add_systems(Update, drop_claw)
        .add_systems(Update, raise_claw)
        .add_systems(Update, open_claw)
        .add_systems(Update, close_claw);
    }
}