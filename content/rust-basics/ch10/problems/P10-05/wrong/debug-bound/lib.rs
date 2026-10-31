#[derive(Debug, PartialEq)]
pub enum Outcome<T, E> {
    Value(T),
    Error(E),
    Empty,
}

pub fn outcome_tag<T: std::fmt::Debug, E>(outcome: &Outcome<T, E>) -> u8 {
    match outcome {
        Outcome::Value(_) => 1,
        Outcome::Error(_) => 2,
        Outcome::Empty => 0,
    }
}
