use bevy::prelude::*;

mod systems;
mod componets;

pub struct TextPlugin;

use systems::*;

impl Plugin for TextPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, move_claw_text)
            .add_systems(Update, despawn_help_text);
    }
}