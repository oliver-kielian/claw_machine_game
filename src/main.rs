use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, move_claw)
    .run();
}

#[derive(Component)]
struct Claw;

#[derive(Component)]
struct BallsGroup;

#[derive(Component)]
struct TopMachine;

#[derive(Component)]
struct RightMachine;

#[derive(Component)]
struct LeftMachine;


fn setup(
    mut commands : Commands,
    asset_server: Res<AssetServer>,
){
    let text_handle_top = asset_server.load("background/topMachine.png");
    let text_handle_bottom = asset_server.load("background/bottomMachine.png");
    let text_handle_right = asset_server.load("background/rightMachine");
    let text_handle_left = asset_server.load("background/leftMachine");
    let text_handle_shadow_balls = asset_server.load("background/shadowBall.png");
    let text_handle_balls_group = asset_server.load("background/ballsGroup.png");

    let text_handle_claw_closed = asset_server.load("background/clawClosed.png");
   

    commands.spawn(Camera2dBundle{
        ..default()
    });
    
    commands.spawn(
        (SpriteBundle{
        texture: text_handle_top,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    },
    TopMachine));


    commands.spawn(SpriteBundle{
        texture: text_handle_shadow_balls,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, 0.0, -2.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(
        (SpriteBundle{
        texture: text_handle_balls_group,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    },
    BallsGroup));

    commands.spawn((SpriteBundle{
        texture: text_handle_left,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    },
    LeftMachine));

    commands.spawn((SpriteBundle{
        texture: text_handle_right,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    },
    RightMachine));


    commands.spawn(SpriteBundle{
        texture: text_handle_bottom,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, -200.0, 1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        SpriteBundle{
        texture: text_handle_claw_closed,
        transform: Transform {
            scale: Vec3::splat(0.2), 
            translation: Vec3::new(0.0, 0.0, -1.0),
            ..default()
        },
        ..default()
    },
    Claw
    ));

}


fn move_claw(mut query: Query<&mut Transform, With<Claw>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>)
{
    let mut tr = query.single_mut();
    let speed = 200.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft){
        tr.translation.x -= speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::ArrowRight){
        tr.translation.x += speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::ArrowDown){
        tr.translation.y -= speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::ArrowUp){
        tr.translation.y += speed * time.delta_seconds();
    }
}
/* 
pub clawColisions(
    mut claw_query: Query<(Entity, &Transform), With<Claw>>,
    mut top_query: Query<&Transform, With<TopMachine>>,
    mut sides_query: Query<&Transform, With<BottomMachine>>,
    mut balls_query: Query<&Transform, with<BallsGroup>>
)
*/