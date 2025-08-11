extern crate sdl2;
mod visuals;
mod input;
use sdl2::keyboard::Keycode;
use sdl2::{rect::Rect, render::Canvas, video::Window, Sdl, VideoSubsystem};
use std::collections::HashSet;
use std::{sync::LazyLock};
use visuals::Display;
use visuals::Dimensions;
use input::Input;
struct Paddle {
    x: f32,
    v: f32
} 

const SCREEN_DIMENSIONS: Dimensions = Dimensions {height: 480, width: 640};
const PADDLE_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 100};
const BALL_DIMENSIONS: Dimensions = Dimensions {height: 20, width: 20};
const BRICK_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 60};
const PADDLE_MAX_SPEED: f32 = 20.0;
const PADDLE_ACCELERATION: f32 = 10.0;
const PADDLE_DECELERATION: f32 = 15.0;
fn main() -> Result<(), String> {
    
    let sdl_context = sdl2::init()?;
    let mut display = Display::new(&sdl_context, SCREEN_DIMENSIONS, PADDLE_DIMENSIONS, BALL_DIMENSIONS, BRICK_DIMENSIONS)?;
    let mut input = Input::new(&sdl_context, SCREEN_DIMENSIONS.width - PADDLE_DIMENSIONS.width,
                                                        0, HashSet::from_iter([Keycode::Right]),
                                                        HashSet::from_iter([Keycode::Left]), PADDLE_MAX_SPEED, PADDLE_ACCELERATION,
                                                        PADDLE_DECELERATION)?;
    let mut game_over = false;

    loop {
        if game_over {
            break;
        }
        input.handle_input(0.1, || {game_over = true;});
        display.clear();
        display.draw_paddle(input.get_position());
        display.show();

    }
    return Ok(());
}
