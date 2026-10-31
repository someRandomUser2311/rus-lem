#[derive(Debug, PartialEq)]
pub enum Outcome<T, E> {
    Value(T),
    Error(E),
    Empty,
}

pub fn outcome_tag<T, E>(outcome: &Outcome<T, E>) -> u8 {
    match outcome {
        Outcome::Value(_) => 2,
        Outcome::Error(_) => 1,
        Outcome::Empty => 0,
    }
}
