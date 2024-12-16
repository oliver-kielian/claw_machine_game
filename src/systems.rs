use bevy::prelude::*;

use crate::game::claw::componets::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_handle_top = asset_server.load("background/topMachine.png");
    let text_handle_bottom = asset_server.load("background/bottomMachine.png");
    let text_handle_right = asset_server.load("background/rightMachine.png");
    let text_handle_left = asset_server.load("background/leftMachine.png");
    let text_handle_shadow_balls = asset_server.load("background/shadowBall.png");
    let text_handle_balls_group = asset_server.load("background/ballsGroup.png");

    let text_handle_claw_closed = asset_server.load("background/clawClosed.png");

    commands.spawn(Camera2dBundle { ..default() });
    
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/music.mp3"),
        settings: PlaybackSettings::LOOP,
    });

    commands.spawn((
        SpriteBundle {
        texture: text_handle_top,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, 280.0, 3.0),
            ..default()
        },
        ..default()
    },
        TopMachine,
    ));
 
    commands.spawn(SpriteBundle {
        texture: text_handle_shadow_balls,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, -45.0, -5.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        SpriteBundle {
        texture: text_handle_balls_group,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, -45.0, -1.0),
            ..default()
        },
        ..default()
    },
        BallsGroup,
    ));

    commands.spawn((
        SpriteBundle {
        texture: text_handle_left,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(-245.0, 35.0, 2.0),
            ..default()
        },
        ..default()
    },
        LeftMachine,
    ));

    commands.spawn((
        SpriteBundle {
        texture: text_handle_right,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(247.0, 30.0, 2.0),
            ..default()
        },
        ..default()
    },
        RightMachine,
    ));

    commands.spawn(SpriteBundle {
        texture: text_handle_bottom,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, -220.0, 1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        SpriteBundle {
        texture: text_handle_claw_closed,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, 200.0, -2.0),
            ..default()
        },
        ..default()
    },
        Claw,
    ));
}