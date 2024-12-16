use bevy::prelude::*;

use super::componets::*;
use super::resources::*;

const X_MAX: f32 = 150.0;
const X_MIN: f32 = -150.0;
const Y_MAX: f32 = 200.0;
const Y_MIN: f32 = 60.0;
const SPEED: f32 = 200.0;

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