pub trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> u64;
    fn duplicate(&self) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub side: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub w: u64,
    pub h: u64,
}

impl Shape for Square {
    fn name(&self) -> String {
        String::from("square")
    }

    fn area(&self) -> u64 {
        self.side.saturating_mul(self.side)
    }

    fn duplicate(&self) -> Self {
        self.clone()
    }
}

impl Shape for Rect {
    fn name(&self) -> String {
        String::from("rect")
    }

    fn area(&self) -> u64 {
        self.w.saturating_mul(self.h)
    }

    fn duplicate(&self) -> Self {
        self.clone()
    }
}

pub fn total_area(shapes: &[Box<dyn Shape>]) -> u64 {
    shapes.iter().fold(0u64, |t, s| t.wrapping_add(s.area()))
}

pub fn pair<T: Shape>(shape: &T) -> (T, T) {
    (shape.duplicate(), shape.duplicate())
}
