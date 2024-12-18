use bevy::prelude::*;

pub mod ball;
pub mod cat;
pub mod claw;
pub mod text;

use crate::game::ball::BallPlugin;
use crate::game::cat::CatPlugin;
use crate::game::claw::ClawPlugin;
use crate::game::text::TextPlugin;

///Plugin to allow for better orginization
pub struct GamePlugin;

///Impliments Plugin trait for TextPlugin
///Insets the relevant systems and resources to the Game logic of the app
impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(BallPlugin)
            .add_plugins(CatPlugin)
            .add_plugins(ClawPlugin)
            .add_plugins(TextPlugin);
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    use ball::componets::BallTimer;
    use ball::resources::BallState;
    use bevy::time::TimePlugin;
    use claw::resources::ClawState;
    use rand::Rng;

    #[test]
    pub fn test_ball_timer(){

        let mut rng = rand::thread_rng();
        let random_time = rng.gen_range(1.0..5.0);

        let mut app = App::new();
            app.add_plugins(TimePlugin);
            let claw_state = ClawState {
                is_moving: false,
                down: true,
                up: false,
            };
            app.insert_resource(BallState::default());

            
            let mut ball_state = app.world_mut().resource_mut::<BallState>();
            //Simulate ball_spawn
            if claw_state.down && !claw_state.is_moving && !ball_state.is_attached{
                ball_state.is_attached = true;
                drop(ball_state);
                app.world_mut().spawn(BallTimer(Timer::from_seconds(random_time, TimerMode::Once)));
            }

            let ball_state = app.world().resource::<BallState>();
            //simulate move_ball(this included timer ticking)
            if ball_state.is_attached{
            let mut i = 0.0;
            while i < random_time{
                let delta = std::time::Duration::from_secs_f32(0.5);
                app.update();

                let mut query = app.world_mut().query::<&mut BallTimer>();
                for mut timer in query.iter_mut(&mut app.world_mut()) {
                    println!("Step {}: Timer before tick: {:?}", i, timer.0);
                    timer.0.tick(delta);
                    println!("Step {}: Timer after tick: {:?}", i, timer.0);
        
                    if timer.0.finished() {
                        let epsilon = 0.0001;
                        assert!((timer.0.elapsed_secs() - random_time).abs() < epsilon, 
                        "Timer's elapsed time is not equal to random_time");
                    }
                }
                i += 1.0;
            }
        }
    }
}