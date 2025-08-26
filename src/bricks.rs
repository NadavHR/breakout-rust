use std::cmp::{max, min};

use crate::{clamp, util::Dimensions};




pub struct Bricks {
    display_dimensions: Dimensions,
    gameplay_dimensions: Dimensions,
    brick_dimensions: Dimensions,
    gameplay_to_disp: (f32, f32),
    brick_initial_life: u8,
    buffer: Box<[u8]>,
    pub score: u32
}

impl Bricks {
    pub fn new(display_dimensions: Dimensions, gameplay_dimensions: Dimensions, brick_initial_life: u8, brick_dimensions: Dimensions) -> Bricks {
        let buffer = (0..gameplay_dimensions.area()).map(|_| brick_initial_life).collect();
        let gameplay_to_disp = (
            (display_dimensions.width ) as f32 / gameplay_dimensions.width as f32,
            (display_dimensions.height) as f32 / gameplay_dimensions.height as f32
        ); 
        return Bricks { display_dimensions, gameplay_dimensions, brick_initial_life, buffer, gameplay_to_disp, brick_dimensions, score: 0 }
    }

    fn buffer_index(&self, x: u32, y: u32) -> usize {
        return (y as usize * self.gameplay_dimensions.width as usize) + x as usize;
    }
    
    fn gameplay_to_display(&self, pos: (u32, u32)) -> (u32, u32) {
        return ((pos.0 as f32 * self.gameplay_to_disp.0) as u32, (pos.1 as f32 * self.gameplay_to_disp.1) as u32);
    }

    fn display_to_gameplay(&self,  pos: (u32, u32)) -> (u32, u32) {
        return ((pos.0 as f32 / self.gameplay_to_disp.0).floor() as u32, (pos.1 as f32 / self.gameplay_to_disp.1).floor() as u32);
    }

    pub fn draw_bricks<F: FnMut((u32, u32), u8) >(&self, mut drawing_function: F) {
        for i in 0..self.gameplay_dimensions.width {
            for j in 0..self.gameplay_dimensions.height {
                drawing_function(self.gameplay_to_display((i, j)), self.buffer[self.buffer_index(i, j)]);
            }
        }
    }

    fn calc_collision_against_brick(&mut self, delta_time_sec: f32, calculated_ball_last_pos: (f32, f32), ball_speed: (f32, f32), brick_gameplay_pos: (u32, u32)) ->  Option<(f32, f32)> {
        let brick_base = self.gameplay_to_display(brick_gameplay_pos);
        let buffer_index = self.buffer_index(brick_gameplay_pos.0, brick_gameplay_pos.1);
        if self.buffer[buffer_index] == 0 { // if colliding with already destroyed brick 
            return None;
        }
        // start by assuming successful collision and fix later if wrong
        self.buffer[buffer_index] -= 1;
        self.score += 1; 
        // here we calculate whether the collision was on the x or y axis by checking if it intersected with the x or y of the brick
        let t_y_collision = ((brick_base.1 +
            (if ball_speed.1 < 0.0 {self.brick_dimensions.height} else {0})
            ) as f32 - calculated_ball_last_pos.1) / ball_speed.1;
        if 0.0 <= t_y_collision && t_y_collision <= delta_time_sec { 
            return Some((0.0, (delta_time_sec - t_y_collision) * ball_speed.1));
        }

        let t_x_collision = ((brick_base.0 +
            (if ball_speed.0 < 0.0 {self.brick_dimensions.width} else {0})
            ) as f32 - calculated_ball_last_pos.0) / ball_speed.0;

        if 0.0 <= t_x_collision && t_x_collision <= delta_time_sec {
            return Some(((delta_time_sec - t_x_collision) * ball_speed.0, 0.0));
        }  
        // fix if collision unsuccessful
        self.buffer[buffer_index] += 1; // if the ball was just in the space between bricks, dont decrease bricks life
        self.score -= 1; // if the ball didnt really collide dont increase score
        return None;
    }

    pub fn calc_ball_collision(&mut self, delta_time_sec: f32, ball_pos: (u32, u32), ball_last_pos: (u32, u32), ball_speed: (f32, f32)) -> (f32, f32) {
        let ball_gameplay_pos = self.display_to_gameplay(ball_pos);
        let ball_last_gameplay_pos = (clamp!(self.display_to_gameplay(ball_last_pos).0, (0, self.gameplay_dimensions.width-1)),
                                                clamp!(self.display_to_gameplay(ball_last_pos).1, (0, self.gameplay_dimensions.height-1)));

        let ball_buffer_index = self.buffer_index(ball_gameplay_pos.0, ball_gameplay_pos.1);
        if ball_buffer_index >= self.gameplay_dimensions.area() {  // if not colliding with anything 
            return (0.0, 0.0);
        }
        let calculated_ball_last_pos = (ball_pos.0 as f32 - (ball_speed.0 * delta_time_sec), ball_pos.1 as f32 - (ball_speed.1 * delta_time_sec));

        // check collisions against every brick the ball could have touched on the way 
        for i in min(ball_gameplay_pos.0, ball_last_gameplay_pos.0)..=max(ball_gameplay_pos.0, ball_last_gameplay_pos.0) {
            for j in min(ball_gameplay_pos.1, ball_last_gameplay_pos.1)..=max(ball_gameplay_pos.1, ball_last_gameplay_pos.1) {
                if let Some(penetration) = self.calc_collision_against_brick(delta_time_sec, calculated_ball_last_pos, ball_speed, (i, j)) {
                    return penetration;
                }
            }
        }
        
        return (0.0, 0.0); // there still is the possibility the ball is in a dead space between the bricks 
        
    }
}