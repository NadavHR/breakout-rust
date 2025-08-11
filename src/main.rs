extern crate sdl2;
mod visuals;
mod input;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time;
use visuals::Display;
use visuals::Dimensions;
use input::Input;

const SCREEN_DIMENSIONS: Dimensions = Dimensions {height: 480, width: 640};
const PADDLE_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 100};
const BALL_DIMENSIONS: Dimensions = Dimensions {height: 20, width: 20};
const BRICK_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 60};
const PADDLE_MAX_SPEED: f32 = 2000.0;
const PADDLE_ACCELERATION: f32 = 1800.0;
const PADDLE_DECELERATION: f32 = 1400.0;
const FPS: f32 = 60.0;
const FRAME_TIME_SEC: f32 = 1.0 / FPS;
const MILLIS_TO_SEC: f32 = 0.001;
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
    let mut game_over = false;
    let mut last_time_millis = get_time_millis();
    loop {
        if game_over {
            break;
        }
        let cur_time_millis = get_time_millis();

        let delta_time_sec = (cur_time_millis - last_time_millis) as f32 * MILLIS_TO_SEC;
        if delta_time_sec >= FRAME_TIME_SEC {
            input.handle_input(delta_time_sec, || {game_over = true;});
            display.clear();
            display.draw_paddle(input.get_position());
            display.show();
            last_time_millis = cur_time_millis;
        }

    }
    return Ok(());
}
