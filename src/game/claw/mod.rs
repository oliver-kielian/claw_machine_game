use bevy::prelude::*;

pub mod componets;
mod systems;
pub mod resources;

use systems::*;
use resources::*;

///Plugin to allow for better orginization
pub struct ClawPlugin;


///Impliments Plugin trait for Claw
/// Insets the relevant systems and resources to the app
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