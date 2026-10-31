#[derive(Debug, PartialEq)]
pub struct Pair<T, U> {
    pub x: T,
    pub y: U,
}

pub fn pair<T: Clone, U: Clone>(x: T, y: U) -> Pair<T, U> {
    Pair { x, y }
}
