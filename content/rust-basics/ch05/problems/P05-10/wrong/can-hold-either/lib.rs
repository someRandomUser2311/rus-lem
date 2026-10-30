#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    pub fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width || self.height > other.height
    }
}
