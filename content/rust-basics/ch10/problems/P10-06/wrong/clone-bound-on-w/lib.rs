#[derive(Debug, PartialEq)]
pub struct Pair<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Pair<T, U> {
    pub fn mix<V, W: Clone>(self, other: Pair<V, W>) -> Pair<T, W> {
        Pair {
            x: self.x,
            y: other.y,
        }
    }
}
