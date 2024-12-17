use::bevy::prelude::*;

use super::componets::*;

///Function that spawns helpful text on how to play the game
pub fn move_claw_text(
    mut commands: Commands
){
    commands.spawn((Text2dBundle{
        text: Text::from_section("Press the left arrow\nto move the claw to the left\n\nPress the right arrow\nto move the claw to the right", 
    TextStyle::default()),
        transform: Transform{
            translation: Vec3::new(-500., 300., 1.),
            ..default()
        },
      ..default()
    },
    HelpTextLeftRight
    ));

    commands.spawn((Text2dBundle{
    text: Text::from_section("Press Space\nto drop and raise the claw", TextStyle::default()),
    transform: Transform{
        translation: Vec3::new(-500., 100., 1.),
        ..default()
    },
    ..default()
    },
    HelpTextSpace
    ));

    commands.spawn((Text2dBundle{
        text: Text::from_section("You have anywhere from 1 to 5 seconds\nto drop the ball into the gift return\n Good Luck!", 
    TextStyle::default()),
    transform: Transform{
        translation: Vec3::new(-500., -100., 1.),
        ..default()
    },
    ..default()
    },
    HelpTextSpace
    ));

}

///Function that depsawns that text are keybaord inputs
/// If the left or right arrows keys or space bar are pressed then the text despawns.
pub fn despawn_help_text(
    mut commands: Commands,
    query_side: Query<Entity, With<HelpTextLeftRight>>,
    query_vertical: Query<Entity, With<HelpTextSpace>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::Space){
        for text in query_vertical.iter() {
            commands.entity(text).despawn();
        }
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ArrowLeft){
        for text in query_side.iter() {
            commands.entity(text).despawn();
        }
    }
}