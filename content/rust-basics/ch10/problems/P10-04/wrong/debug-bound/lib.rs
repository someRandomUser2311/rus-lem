#[derive(Debug, PartialEq)]
pub struct Pair<T, U> {
    pub x: T,
    pub y: U,
}

pub fn pair<T: std::fmt::Debug, U>(x: T, y: U) -> Pair<T, U> {
    Pair { x, y }
}
