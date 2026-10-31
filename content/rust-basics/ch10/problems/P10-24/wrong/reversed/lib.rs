#[derive(Debug, PartialEq)]
pub enum Outcome<T, E> {
    Value(T),
    Error(E),
    Empty,
}

pub fn partition<T, E>(outcomes: Vec<Outcome<T, E>>) -> (Vec<T>, Vec<E>, usize) {
    let mut values = Vec::new();
    let mut errors = Vec::new();
    let mut empty = 0;
    for outcome in outcomes {
        match outcome {
            Outcome::Value(value) => values.push(value),
            Outcome::Error(error) => errors.push(error),
            Outcome::Empty => empty += 1,
        }
    }
    values.reverse();
    (values, errors, empty)
}
