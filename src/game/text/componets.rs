use::bevy::prelude::*;
///Compentent to control text for moving the claw to the left and right
#[derive(Component)]
pub struct HelpTextLeftRight;

///Compentent to control text for moving the claw to up and down.
///Also controls teh text providing information on how long the Claw will hold the Ball
#[derive(Component)]
pub struct HelpTextSpace;