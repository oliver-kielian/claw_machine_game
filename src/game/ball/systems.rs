use bevy::prelude::*;
use rand::Rng;

use super::componets::*;
use super::resources::*;

use crate::game::claw::componets::*;
use crate::game::claw::resources::*;

const BALLOFFSET: f32 = 90.0;
const SPEED: f32 = 200.0;

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

            let mut rng = rand::thread_rng();
            let random_time = rng.gen_range(1.0..5.0);

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
                BallTimer(Timer::from_seconds(random_time, TimerMode::Once))
            ));

            ball_state.is_attached = true;
        }
    }

}

pub fn move_ball(
    mut ball_query: Query<(&mut Transform, &mut BallTimer), (With<Ball>, Without<Claw>)>,
    claw_query: Query<&Transform, With<Claw>>,
    time: Res<Time>,
    mut ball_state: ResMut<BallState>
) {
    for (mut ball_transform, mut timer) in ball_query.iter_mut() {
        if let Ok(claw_transform) = claw_query.get_single() {
            if ball_state.is_attached {
                ball_transform.translation.x = claw_transform.translation.x;
                ball_transform.translation.y = claw_transform.translation.y - BALLOFFSET;

                timer.0.tick(time.delta());
                if timer.0.finished() {
                    ball_state.is_attached = false;
                }
            }
        }
    }
}

pub fn drop_ball(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Transform), With<Ball>>,
    time: Res<Time>,
    ball_state: Res<BallState>,
    mut game: ResMut<Game>
) {
    if !ball_state.is_attached {
        for (ball, mut transform) in ball_query.iter_mut() {
            transform.translation.y -= SPEED * time.delta_seconds();
            transform.translation.z = 0.0;

            if transform.translation.y <= -240.0{
                transform.translation.y = -240.0;
                if transform.translation.x >= -235.0 && transform.translation.x <= -30.0
                {
                    game.win = true;
                }
                commands.entity(ball).despawn();
            }
        }
    }
}