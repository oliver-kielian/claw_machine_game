use bevy::prelude::*;

pub mod ball;
pub mod cat;
pub mod claw;
pub mod text;

use crate::ball::BallPlugin;
use crate::cat::CatPlugin;
use crate::claw::ClawPlugin;
use crate::text::TextPlugin;

mod systems;

use crate::systems::setup;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .insert_resource(ClearColor(Color::srgb(0.768, 0.643, 0.518)))
    .add_systems(Startup, setup)
    .add_plugins(BallPlugin)
    .add_plugins(CatPlugin)
    .add_plugins(ClawPlugin)
    .add_plugins(TextPlugin)
    .run();
}