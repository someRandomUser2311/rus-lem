#[derive(Debug, PartialEq)]
pub struct Pair<T, U> {
    pub x: T,
    pub y: U,
}

impl<T: PartialOrd + Copy> Pair<T, T> {
    pub fn smaller(&self) -> &T {
        if self.x <= self.y { &self.x } else { &self.y }
    }
}
