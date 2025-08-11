extern crate sdl2;
mod visuals;
use sdl2::{rect::Rect, render::Canvas, video::Window, Sdl, VideoSubsystem};
use std::{sync::LazyLock};
use visuals::Display;
use visuals::Dimensions;
struct Paddle {
    x: f32,
    v: f32
} 

const SCREEN_DIMENSIONS: Dimensions = Dimensions {height: 480, width: 640};
const PADDLE_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 100};
const BALL_DIMENSIONS: Dimensions = Dimensions {height: 20, width: 20};
const BRICK_DIMENSIONS: Dimensions = Dimensions {height: 30, width: 60};

fn main() -> Result<(), String> {
    
    let sdl_context = sdl2::init()?;
    let mut display = Display::new(&sdl_context, SCREEN_DIMENSIONS, PADDLE_DIMENSIONS, BALL_DIMENSIONS, BRICK_DIMENSIONS)?;
    let mut lost = false;

    loop {
        if lost {
            break;
        }
        display.clear();
        display.draw_paddle(0);
        display.show();

    }
    return Ok(());
}
