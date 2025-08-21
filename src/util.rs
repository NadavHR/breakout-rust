#[derive(Clone, Copy)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32
}

impl Dimensions {
    pub fn area(&self) -> usize{
        self.height as usize * self.width as usize
    }
}