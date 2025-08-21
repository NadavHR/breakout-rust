
use derive_more::Constructor;
use crate::util::Dimensions;

macro_rules! collide {
    ($pos:expr, $speed:expr, $bound_hit:expr ) => {
        $speed *= -1.0;
        $pos = ($bound_hit) - ($pos - ($bound_hit));
    };
}

#[derive(Constructor)]
pub struct Ball {
    x: f32,
    y: f32,
    
    x_speed: f32,
    y_speed: f32,
    
    y_max: u32,
    y_min: u32,

    x_max: u32,
    x_min: u32,

    x_max_speed: f32,
    x_speed_gain: f32,

    ball_dimensions: Dimensions,
    paddle_dimension: Dimensions,

}

// TODO: add ball collision with bricks
impl Ball {
    fn bounds_collision_x(&mut self) {
        // yes how this is calculated does mean that if the ball gets too fast it will always remain offscreen but the ball has a max speed.
        if self.x > self.x_max as f32{
            collide!(self.x, self.x_speed, self.x_max as f32);
        } else if self.x < self.x_min as f32{
            collide!(self.x, self.x_speed, self.x_min as f32);
        }
    } 

    fn bounds_collision_y(&mut self, paddle_x: u32, paddle_speed: f32, mut end_game: impl FnMut()) {
        let paddle_y = (self.y_max - self.paddle_dimension.height) as f32;
        if self.y < self.y_min as f32 {
            collide!(self.y, self.y_speed, self.y_min as f32);
        } else if self.y > paddle_y { // either collision with paddle or game over
            if (paddle_x..(paddle_x + self.paddle_dimension.width)).contains(&(self.x as u32)) && self.y_speed > 0.0 { // bounced
                // collide!(self.y, self.y_speed, paddle_y);
                self.y_speed *= -1.0;
                self.x_speed = (paddle_speed + (self.x_speed * self.x_speed_gain)).clamp(-self.x_max_speed, self.x_max_speed);
            } else if self.y > self.y_max as f32 { // game over 
                end_game();
            } 
        }
    }


    pub fn update(&mut self, delta_time: f32, paddle_x: u32, paddle_speed: f32, end_game: impl FnMut()) {
        
        self.x += self.x_speed * delta_time;
        self.y += self.y_speed * delta_time;

        // x collision with screen bounds
        self.bounds_collision_x();

        // y collisions with screen bounds and paddle
        self.bounds_collision_y(paddle_x, paddle_speed, end_game);

    }

    pub fn get_ball_coords(&self) -> (u32, u32) {
        return ((self.x - (self.ball_dimensions.width as f32 * 0.5)) as u32,
                (self.y - (self.ball_dimensions.height as f32 * 0.5)) as u32);
    }
}