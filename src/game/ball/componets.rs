use bevy::prelude::*;
///Struct that acts as a component to query all objects within the game as Ball components. Used for the physical ball that holds the winnable.
#[derive(Component)]
pub struct Ball;

///Controls when the ball falls from the claw's hold
#[derive(Component)]
pub struct BallTimer(pub Timer);