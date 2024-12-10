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


fn setup(
    mut commands : Commands,
    asset_server: Res<AssetServer>,
){
    let text_handle_top = asset_server.load("background/topMachine.png");
    let text_handle_bottom = asset_server.load("background/bottomMachine.png");
    let text_handle_shadow_balls = asset_server.load("background/shadowBall.png");
    let text_handle_balls_group = asset_server.load("background/ballsGroup.png");

    let text_handle_claw_closed = asset_server.load("background/clawClosed.png");
   

    commands.spawn(Camera2dBundle{
        ..default()
    });
    commands.spawn(SpriteBundle{
        texture: text_handle_top,
        sprite: Sprite {
            custom_size: Some(Vec2::new(700.0, 700.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    });


    commands.spawn(SpriteBundle{
        texture: text_handle_shadow_balls,
        sprite: Sprite {
            custom_size: Some(Vec2::new(700.0, 700.0)), 
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -2.0)),
        ..default()
    });

    commands.spawn(SpriteBundle{
        texture: text_handle_balls_group,
        sprite: Sprite {
            custom_size: Some(Vec2::new(700.0, 700.0)), 
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    });

    commands.spawn(SpriteBundle{
        texture: text_handle_bottom,
        sprite: Sprite {
            custom_size: Some(Vec2::new(700.0, 700.0)), 
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    });

    commands.spawn((
        SpriteBundle{
        texture: text_handle_claw_closed,
        sprite: Sprite {
            custom_size: Some(Vec2::new(700.0, 700.0)), 
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
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
