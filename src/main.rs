extern crate sdl2;
mod visuals;
mod input;
mod ball;
mod util;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time;
use visuals::Display;
use input::Input;
use ball::Ball;
use util::Dimensions;

const SCREEN_DIMENSIONS: Dimensions = Dimensions {height: 480, width: 640};
const PADDLE_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 100};
const BALL_DIMENSIONS: Dimensions = Dimensions {height: 10, width: 10};
const BRICK_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 60};
const PADDLE_MAX_SPEED: f32 = 2000.0;
const PADDLE_ACCELERATION: f32 = 1800.0;
const PADDLE_DECELERATION: f32 = 1400.0;
const FPS: f32 = 60.0;
const FRAME_TIME_SEC: f32 = 1.0 / FPS;
const MILLIS_TO_SEC: f32 = 0.001;
const BALL_Y_SPEED: f32 = 100.0; 
fn get_time_millis() -> u128 {
    return time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis();
}
fn main() -> Result<(), String> {
    
    let sdl_context = sdl2::init()?;
    let mut display = Display::new(&sdl_context, SCREEN_DIMENSIONS, PADDLE_DIMENSIONS, BALL_DIMENSIONS, BRICK_DIMENSIONS)?;
    let mut input = Input::new(&sdl_context, SCREEN_DIMENSIONS.width - PADDLE_DIMENSIONS.width,
                                                        0, HashSet::from_iter([Keycode::Right]),
                                                        HashSet::from_iter([Keycode::Left]), PADDLE_MAX_SPEED, PADDLE_ACCELERATION,
                                                        PADDLE_DECELERATION)?;
    let mut ball = Ball::new((SCREEN_DIMENSIONS.width as f32) * 0.5, (SCREEN_DIMENSIONS.height as f32) * 0.5, 
                                0.0, BALL_Y_SPEED, SCREEN_DIMENSIONS.height, 0, SCREEN_DIMENSIONS.width,
                                0, BALL_DIMENSIONS, PADDLE_DIMENSIONS);
    let mut game_over = false;
    let mut last_time_millis = get_time_millis();
    loop {
        if game_over {
            break;
        }
        let cur_time_millis = get_time_millis();

        let delta_time_sec = (cur_time_millis - last_time_millis) as f32 * MILLIS_TO_SEC;
        let mut end_game = || {game_over = true;};
        if delta_time_sec >= FRAME_TIME_SEC {
            input.handle_input(delta_time_sec,&mut end_game);
            ball.update(delta_time_sec, input.get_position(), input.get_paddle_speed(), &mut end_game);
            display.clear();
            display.draw_paddle(input.get_position());
            display.draw_ball(ball.get_ball_coords());
            display.show();
            last_time_millis = cur_time_millis;
        }

    }
    return Ok(());
}