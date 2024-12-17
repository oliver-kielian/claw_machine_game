use bevy::prelude::*;

mod game;
mod main_menu;

use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;

mod systems;
use crate::systems::setup;

fn main() {
    //Creation of the app with all Plugins
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(GamePlugin)
    .add_plugins(MainMenuPlugin)
    .insert_resource(ClearColor(Color::srgb(0.768, 0.643, 0.518)))
    .add_systems(Startup, setup)
    .run();
}