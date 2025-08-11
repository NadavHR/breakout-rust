use std::collections::HashSet;

use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

pub struct Input<'a> {
    sdl_context: &'a Sdl,
    events: EventPump,
    paddle_max_x: u32,
    paddle_min_x: u32,
    paddle_max_speed: f32,
    paddle_acceleration: f32,
    paddle_deceleration: f32,
    paddle_position: f32,
    paddle_velocity: f32,
}

impl Input<'_> {
    pub fn new<'a>(
        sdl_context: &'a Sdl,
        paddle_max_x: u32,
        paddle_min_x: u32,
        paddle_max_speed: f32,
        paddle_acceleration: f32,
        paddle_deceleration: f32,
    ) -> Result<Input<'a>, String> {
        return Ok(Input {
            sdl_context, paddle_max_x, paddle_min_x, paddle_max_speed, paddle_acceleration, paddle_deceleration, 
            events: sdl_context.event_pump()?, paddle_position: 0.0, paddle_velocity: 0.0
        })
    }
    pub fn handle_input(&mut self, delta_time_sec: f32, case_of_quit: impl Fn()) {
        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                case_of_quit();
            };
        }
        let keys: HashSet<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        
    }
}
