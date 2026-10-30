#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: height,
            height: width,
        }
    }

    pub fn square(size: u32) -> Self {
        Self::new(size, size)
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
