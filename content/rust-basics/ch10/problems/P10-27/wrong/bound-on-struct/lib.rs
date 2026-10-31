#[derive(Debug, PartialEq)]
pub struct Pair<T: Ranked, U> {
    pub x: T,
    pub y: U,
}

pub trait Ranked {
    fn points(&self) -> i64;
}

pub struct Score {
    pub label: String,
    pub points: i64,
}

impl Ranked for Score {
    fn points(&self) -> i64 {
        self.points
    }
}

impl<T: Ranked> Pair<T, T> {
    pub fn winner(&self) -> &T {
        if self.x.points() >= self.y.points() {
            &self.x
        } else {
            &self.y
        }
    }
}
