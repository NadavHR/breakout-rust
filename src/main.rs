extern crate sdl2;
mod visuals;
mod input;
mod ball;
mod util;
mod bricks;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time;
use visuals::Display;
use input::Input;
use ball::Ball;
use util::Dimensions;

use crate::bricks::Bricks;

const SCREEN_DIMENSIONS: Dimensions = Dimensions {height: 480, width: 640};
const PADDLE_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 100};
const BALL_DIMENSIONS: Dimensions = Dimensions {height: 8, width: 8};
const BRICK_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 70};
const BRICKS_GAMEPLAY_DIMENSIONS: Dimensions = Dimensions {
    height: 6, 
    width: SCREEN_DIMENSIONS.width / (BRICK_DIMENSIONS.width + 10) // we want 10 pixel gap between brick in both x and y 
}; 
const BRICKS_DISPLAY_DIMENSIONS: Dimensions = Dimensions {
    height: BRICKS_GAMEPLAY_DIMENSIONS.height * (BRICK_DIMENSIONS.height + 10), // we want 10 pixel gap between brick in both x and y 
    width: SCREEN_DIMENSIONS.width
};
const PADDLE_MAX_SPEED: f32 = 2000.0;
const PADDLE_ACCELERATION: f32 = 1800.0;
const PADDLE_DECELERATION: f32 = 1400.0;
const FPS: f32 = 60.0;
const FRAME_TIME_SEC: f32 = 1.0 / FPS;
const MILLIS_TO_SEC: f32 = 0.001;
const BALL_Y_SPEED: f32 = 100.0; 
const BALL_MAX_SPEED_X: f32 = 0.2 * PADDLE_MAX_SPEED;
const BALL_X_SPEED_GAIN: f32 = 0.5;
const BRICK_LIFE: u8 = 3;

fn get_time_millis() -> u128 {
    return time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis();
}
fn main() -> Result<(), String> {
    
    let sdl_context = sdl2::init()?;
    let mut display = Display::new(&sdl_context, SCREEN_DIMENSIONS,
         PADDLE_DIMENSIONS, BALL_DIMENSIONS, BRICK_DIMENSIONS, BRICK_LIFE)?;
    let mut input = Input::new(&sdl_context, SCREEN_DIMENSIONS.width - PADDLE_DIMENSIONS.width,
                                                        0, HashSet::from_iter([Keycode::Right]),
                                                        HashSet::from_iter([Keycode::Left]), PADDLE_MAX_SPEED, PADDLE_ACCELERATION,
                                                        PADDLE_DECELERATION)?;
    let mut ball = Ball::new((SCREEN_DIMENSIONS.width as f32) * 0.5, (SCREEN_DIMENSIONS.height - PADDLE_DIMENSIONS.height)as f32, 
                                0.0, -BALL_Y_SPEED, SCREEN_DIMENSIONS.height, 0, SCREEN_DIMENSIONS.width,
                                0, BALL_MAX_SPEED_X, BALL_X_SPEED_GAIN, PADDLE_DIMENSIONS);
    let mut bricks = Bricks::new(BRICKS_DISPLAY_DIMENSIONS, BRICKS_GAMEPLAY_DIMENSIONS, BRICK_LIFE, BRICK_DIMENSIONS);
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
            ball.apply_collision(
                bricks.calc_ball_collision(delta_time_sec,
                    ball.get_ball_coords(), ball.get_last_ball_pos(), ball.get_ball_speed()));
            bricks.draw_bricks(|pos: (u32, u32), life: u8| display.draw_brick(pos, life));
            display.draw_ball(ball.get_ball_coords());
            display.set_score(bricks.score).unwrap();
            display.show();
            last_time_millis = cur_time_millis;
            if bricks.score == BRICKS_GAMEPLAY_DIMENSIONS.area() as u32 * BRICK_LIFE as u32 {
                game_over = true;
                println!("You Won!");
            }
        }
    }
    println!("SCORE: {}", bricks.score);
    return Ok(());
}