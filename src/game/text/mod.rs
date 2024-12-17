use bevy::prelude::*;

mod systems;
mod componets;

///Plugin to allow for better orginization
pub struct TextPlugin;

use systems::*;

///Impliments Plugin trait for TextPlugin
///Insets the relevant systems and resources to the app
impl Plugin for TextPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, move_claw_text)
            .add_systems(Update, despawn_help_text);
    }
}