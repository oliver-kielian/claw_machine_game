use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}


fn setup(
    mut commands : Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
){
    let text_handle_top = asset_server.load("background/topMachine.png");
    let text_handle_bottom = asset_server.load("background/bottomMachine.png");
    let text_handle_shadow_balls = asset_server.load("background/shadowBall.png");
    let text_handle_balls_group = asset_server.load("background/ballsGroup.png");

    let text_handle_claw = asset_server.load("background/claws.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64,128), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);


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

    commands.spawn(SpriteBundle{
        texture: text_handle_claw,
        sprite: Sprite {
            custom_size: Some(Vec2::new(700.0, 700.0)), 
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        ..default()
    });

}
