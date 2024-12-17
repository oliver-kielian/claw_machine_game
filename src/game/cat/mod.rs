use bevy::prelude::*;

mod systems;
mod componets;

use systems::*;

///Plugin to allow for better orginization
pub struct CatPlugin;

///Impliments Plugin trait for CatPlugin
/// Insets the relevant systems to the app
impl Plugin for CatPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, win_cat)
            .add_systems(Update, depawn_cat)
            .add_systems(Update, animate_cat);
    }
}