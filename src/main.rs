use bevy::prelude::*;
use rand::Rng;

const X_MAX: f32 = 150.0;
const X_MIN: f32 = -150.0;
const Y_MAX: f32 = 200.0;
const Y_MIN: f32 = 60.0;
const SPEED: f32 = 200.0;
const BALLOFFSET: f32 = 90.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
     .insert_resource(ClawState::default())
     .insert_resource(BallState::default())
    .add_systems(Startup, setup)
    .add_systems(Update, move_claw)
    .add_systems(Update, claw_collisions)
    .add_systems(Update, drop_claw)
    .add_systems(Update, raise_claw)
    .add_systems(Update, open_claw)
    .add_systems(Update, close_claw)
    .add_systems(Update, spawn_ball)
    .add_systems(Update, move_ball)
    .run();
}

#[derive(Component)]
pub struct Claw;

#[derive(Resource)]
pub struct ClawState {
    is_moving: bool,
    down: bool,
    up: bool,
}

impl Default for ClawState {
    fn default() -> Self {
        ClawState {
            is_moving: false,
            down: false,
            up: true,
        }
    }
}

#[derive(Component)]
struct BallsGroup;

#[derive(Component)]
struct TopMachine;

#[derive(Component)]
struct RightMachine;

#[derive(Component)]
struct LeftMachine;

#[derive(Component)]
pub struct Ball;

#[derive(Resource)]
pub struct BallState {
    is_attached: bool,
}

impl Default for BallState {
    fn default() -> Self {
        BallState { is_attached: false }
    }
}

#[derive(Component)]
struct BallTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn move_claw(
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

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        tr.translation.y -= SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        tr.translation.y += SPEED * time.delta_seconds();
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

pub fn spawn_ball(
    mut claw_query: Query<&Transform, With<Claw>>,
    claw_state: Res<ClawState>,
    mut ball_state: ResMut<BallState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if claw_state.down && !claw_state.is_moving && !ball_state.is_attached{
        let ball_handle = asset_server.load("background/balls.png");

        let layout = TextureAtlasLayout::from_grid(UVec2::new(332, 408), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let random_index = rand::thread_rng().gen_range(0..6);

        

        if let Ok(claw_transform) = claw_query.get_single_mut() {

            let claw_y = claw_transform.translation.y;
            let claw_x = claw_transform.translation.x;

            commands.spawn((
                SpriteBundle {
                texture: ball_handle,
                transform: Transform {
                    scale: Vec3::splat(0.2), 
                        translation: Vec3::new(claw_x, claw_y-BALLOFFSET, -3.0),
                    ..default()
                },
                ..default()
            },
                TextureAtlas {
                layout: texture_atlas_layout,
                index: random_index,
            },
                Ball,
            ));

            ball_state.is_attached = true;
        }
    }

}

pub fn move_ball(
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<Claw>)>,
    claw_query: Query<&Transform, With<Claw>>,
) {
    // Logic for moving the ball
    for mut ball_transform in ball_query.iter_mut() {
        // Move the ball with the claw's position
        if let Ok(claw_transform) = claw_query.get_single() {
            ball_transform.translation.x = claw_transform.translation.x;
            ball_transform.translation.y = claw_transform.translation.y - BALLOFFSET;
        }
    }
}