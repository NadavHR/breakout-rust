use sdl2::Sdl;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct Input<'a> {
    sdl_context: &'a Sdl,
    paddle_max_x: u32,
    paddle_min_x: u32,
    paddle_max_speed: f32,
    paddle_acceleration: f32,
    paddle_position_f: f32,
}
