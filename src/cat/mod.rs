use bevy::prelude::*;

mod systems;
mod componets;

use systems::*;
pub struct CatPlugin;

impl Plugin for CatPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, win_cat)
            .add_systems(Update, depawn_cat)
            .add_systems(Update, animate_cat);
    }
}