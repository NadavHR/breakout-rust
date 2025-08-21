use crate::util::Dimensions;



pub struct Bricks {
    display_dimensions: Dimensions,
    gameplay_dimensions: Dimensions,
    brick_dimensions: Dimensions,
    gameplay_to_disp: (f32, f32),
    brick_initial_life: u8,
    buffer: Box<[u8]>
}

impl Bricks {
    pub fn new(display_dimensions: Dimensions, gameplay_dimensions: Dimensions, brick_initial_life: u8, brick_dimensions: Dimensions) -> Bricks {
        let buffer = (0..gameplay_dimensions.area()).map(|_| brick_initial_life).collect();
        let gameplay_to_disp = (
            (display_dimensions.width ) as f32 / gameplay_dimensions.width as f32,
            (display_dimensions.height) as f32 / gameplay_dimensions.height as f32
        ); 
        return Bricks { display_dimensions, gameplay_dimensions, brick_initial_life, buffer, gameplay_to_disp, brick_dimensions }
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

    pub fn calc_ball_collision(&mut self, ball_pos: (u32, u32)) -> (u32, u32) {
        let ball_gameplay_pos = self.display_to_gameplay(ball_pos);
        let ball_buffer_index = self.buffer_index(ball_gameplay_pos.0, ball_gameplay_pos.1);
        if ball_buffer_index >= self.gameplay_dimensions.area() {  // if not colliding with anything
            return (0, 0);
        }
        if self.buffer[ball_buffer_index] == 0 {  // if colliding with already destroyed brick
            return (0, 0);
        }

        self.buffer[ball_buffer_index] -= 1; // decrease life of collided brick
        return (0, 0);
        
    }
}