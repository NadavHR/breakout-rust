use std::{collections::HashSet};

use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

pub struct Input<'a> {
    sdl_context: &'a Sdl,
    events: EventPump,
    right_move_keys: HashSet<Keycode>,
    left_move_keys: HashSet<Keycode>,
    paddle_max_x: u32,
    paddle_min_x: u32,
    paddle_max_speed: f32,
    paddle_acceleration: f32,
    paddle_deceleration: f32,
    paddle_position: f32,
    paddle_speed: f32,
}

impl Input<'_> {
    pub fn new<'a>(
        sdl_context: &'a Sdl,
        paddle_max_x: u32,
        paddle_min_x: u32,
        right_move_keys: HashSet<Keycode>,
        left_move_keys: HashSet<Keycode>,
        paddle_max_speed: f32,
        paddle_acceleration: f32,
        paddle_deceleration: f32,
    ) -> Result<Input<'a>, String> {
        return Ok(Input {
            sdl_context, paddle_max_x, paddle_min_x, paddle_max_speed, paddle_acceleration, paddle_deceleration, right_move_keys, left_move_keys,
            events: sdl_context.event_pump()?, paddle_position: 0.0, paddle_speed: 0.0
        })
    }

    fn set_paddle_speed(&mut self, new_speed: f32) {
        self.paddle_speed = new_speed.max(-self.paddle_max_speed).min(self.paddle_max_speed);
    }

    fn set_paddle_position(&mut self, new_pos: f32) {
        self.paddle_position = new_pos.max(self.paddle_min_x as f32).min(self.paddle_max_x as f32);
    }

    pub fn handle_input(&mut self, delta_time_sec: f32, mut case_of_quit: impl FnMut()) {
        // check press
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
        // check right and left movements
        let right_movement = ((&self.right_move_keys - &keys).len() != self.right_move_keys.len()); // right movement
        let left_movement = ((&self.left_move_keys - &keys).len() != self.left_move_keys.len());    // left movement

        // calc delta speed
        let mut paddle_delta_speed: f32 = 0.0;
        if right_movement || left_movement {
            paddle_delta_speed = (right_movement as u32 as f32 - left_movement as u32 as f32) * self.paddle_acceleration * delta_time_sec;
        } else {
            let dir = (self.paddle_speed).signum(); 
            let mag = self.paddle_speed * dir;
            paddle_delta_speed = -dir * mag.min(self.paddle_deceleration * delta_time_sec); 
        }

        // calc new speed and position
        self.set_paddle_speed(self.paddle_speed + (paddle_delta_speed * 0.5));
        self.set_paddle_position(self.paddle_position + (self.paddle_speed * delta_time_sec));
        self.set_paddle_speed(self.paddle_speed + (paddle_delta_speed * 0.5));
        
    }

    pub fn get_position(&self) -> u32 {
        return self.paddle_position as u32;
    }
}
