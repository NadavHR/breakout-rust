
use std::cmp;
use sdl2::{rect::Rect, render::Canvas, video::Window, Sdl, VideoSubsystem};

use crate::util::Dimensions;

pub struct Display<'a> {
    screen_dimensions: Dimensions,
    paddle_rect: Rect,
    ball_rect: Rect,
    brick_rect: Rect,
    brick_max_life: u8,
    sdl_context: &'a Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>
}

impl Display<'_> {
    pub fn new<'a>(sdl_context: &'a Sdl, 
                    screen_dimensions: Dimensions,
                    paddle_dimensions: Dimensions, 
                    ball_dimensions: Dimensions,
                    brick_dimensions: Dimensions,
                    brick_max_life: u8,
                ) -> Result<Display<'a>, String> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("SDL2", screen_dimensions.width, screen_dimensions.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        return Ok(Display {
            screen_dimensions, 
            paddle_rect: Rect::new(0, (screen_dimensions.height - paddle_dimensions.height) as i32, paddle_dimensions.width, paddle_dimensions.height),
            brick_rect: Rect::new(0, 0, brick_dimensions.width, brick_dimensions.height), 
            ball_rect: Rect::new(0, 0, ball_dimensions.width, ball_dimensions.height), sdl_context, video_subsystem, canvas, brick_max_life
        })
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
        self.canvas.clear();
    }

    pub fn draw_paddle(&mut self, x: u32) {
        self.paddle_rect.set_x(cmp::min(x, self.screen_dimensions.width - self.paddle_rect.width()) as i32);
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 0));
        match self.canvas.fill_rect(self.paddle_rect) {
            Ok(_) => {}
            Err(err) => {println!("error: {err}")}
        }
    }

    pub fn draw_ball(&mut self, pos: (i32, i32)) {
        self.ball_rect.set_x(pos.0);
        self.ball_rect.set_y(pos.1);
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 0));
        match self.canvas.fill_rect(self.ball_rect) {
            Ok(_) => {}
            Err(err) => {println!("error: {err}")}
        }
    }

    pub fn draw_brick(&mut self, pos: (u32, u32), life: u8) {
        self.brick_rect.set_x(pos.0 as i32);
        self.brick_rect.set_y(pos.1 as i32);
        let brightness = ((life as f32 / self.brick_max_life as f32) * 255.0) as u8;
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(brightness, brightness, brightness, 0));
        match self.canvas.fill_rect(self.brick_rect) {
            Ok(_) => {}
            Err(err) => {print!("error: {err}")}
        }
    }

    pub fn show(&mut self) {
        self.canvas.present();
    }
}

