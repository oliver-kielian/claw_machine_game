use bevy::prelude::*;

use super::componets::*;
use super::resources::*;
/// The claw is controlled from the center of the sprite 

const X_MAX: f32 = 150.0; //the max x value (how far to the right) the claw can go
const X_MIN: f32 = -150.0; //the min x value (how far to the left) the claw can go
const Y_MAX: f32 = 200.0; //the max y value (how far up) the claw can go
const Y_MIN: f32 = 60.0; //the min y value (how far down) the claw can go
const SPEED: f32 = 200.0; //The speed of the claw and the ball

///Allows user input to move the claw to the left or to the right based on which arrow key is pressed
pub fn move_claw(
    mut query: Query<&mut Transform, With<Claw>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut tr = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        tr.translation.x -= SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        tr.translation.x += SPEED * time.delta_seconds();
    }
}

///Keeps the claw within the bound of the machine based of the constant values found at the top of this file
pub fn claw_collisions(mut claw_query: Query<(Entity, &mut Transform), With<Claw>>) {
    if let Ok((_claw_entity, mut claw_transform)) = claw_query.get_single_mut() {
        let mut translation = claw_transform.translation;

        if translation.x < X_MIN {
            translation.x = X_MIN
        } else if translation.x > X_MAX {
            translation.x = X_MAX;
        }

        if translation.y < Y_MIN {
            translation.y = Y_MIN;
        } else if translation.y > Y_MAX {
            translation.y = Y_MAX;
        }

        claw_transform.translation = translation;
    }
}
///Function that drops the claw to create the illusion of a ball being grabbed.
/// Changes the ClawState::is_moving to true and changes the postion of the claw to the bottom of the machine by updating ClawState::up to false and ClawState::down to true
pub fn drop_claw(
    mut query: Query<(&mut Transform, &Claw)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut claw_state: ResMut<ClawState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && claw_state.up {
        claw_state.is_moving = true;
    }
    if claw_state.is_moving && claw_state.up {
        for (mut transform, _claw) in query.iter_mut() {
        transform.translation.y -= SPEED * time.delta_seconds();
            if transform.translation.y <= Y_MIN {
            transform.translation.y = Y_MIN;
            claw_state.is_moving = false;
            claw_state.down = true;
            claw_state.up = false;
        }
    }
}
}

//Function that raises the claw after being dropped
/// Changes the ClawState::is_moving to true and changes the postion of the claw to the bottom of the machine by updating ClawState::up to true and ClawState::down to false
pub fn raise_claw(
    mut query: Query<(&mut Transform, &Claw)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut claw_state: ResMut<ClawState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && claw_state.down {
        claw_state.is_moving = true;
    }
    if claw_state.is_moving && claw_state.down {
        for (mut transform, _claw) in query.iter_mut() {
        transform.translation.y += SPEED * time.delta_seconds();
            if transform.translation.y >= Y_MAX {
            transform.translation.y = Y_MAX;
            claw_state.is_moving = false;
            claw_state.down = false;
            claw_state.up = true;
        }
    }
}
}


///Function to open the claw, it replaces the current closed sprite with a new open sprite
/// This happens by spawning the new sprite then despawning the old sprite.
pub fn open_claw(
    mut commands: Commands,
    mut claw_query: Query<(Entity, &Transform), With<Claw>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    claw_state: Res<ClawState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && claw_state.up {
        if let Ok((claw_entity, claw_transform)) = claw_query.get_single_mut() {
        let translation = claw_transform.translation;
        let text_handle_open = asset_server.load("background/clawOpen.png");

        commands.spawn((
                SpriteBundle {
                texture: text_handle_open,
                transform: Transform {
                    scale: Vec3::splat(0.2), 
                    translation,
                    ..default()
                },
                ..default()
            },
                Claw,
        ));

        commands.entity(claw_entity).despawn();
    }
}
}

///Function to close the claw, it replaces the current open sprite with a new closed sprite
/// This happens by spawning the new sprite then despawning the old sprite.
pub fn close_claw(
    mut commands: Commands,
    mut claw_query: Query<(Entity, &Transform), With<Claw>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    claw_state: Res<ClawState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && claw_state.down {
        if let Ok((claw_entity, claw_transform)) = claw_query.get_single_mut() {
        let translation = claw_transform.translation;
        let text_handle_close = asset_server.load("background/clawClosed.png");

        commands.spawn((
                SpriteBundle {
                texture: text_handle_close,
                transform: Transform {
                    scale: Vec3::splat(0.2), 
                    translation,
                    ..default()
                },
                ..default()
            },
                Claw,
        ));

        commands.entity(claw_entity).despawn();
    }
}
}