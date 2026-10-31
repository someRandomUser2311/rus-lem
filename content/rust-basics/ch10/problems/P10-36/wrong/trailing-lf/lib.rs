#[derive(Debug, PartialEq)]
pub enum Outcome<T, E> {
    Value(T),
    Error(E),
    Empty,
}

pub struct Ledger<T, E> {
    pub entries: Vec<Outcome<T, E>>,
}

impl<T, E> Ledger<T, E> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn value(&mut self, value: T) {
        self.entries.push(Outcome::Value(value));
    }

    pub fn error(&mut self, error: E) {
        self.entries.push(Outcome::Error(error));
    }

    pub fn empty(&mut self) {
        self.entries.push(Outcome::Empty);
    }

    pub fn into_parts(self) -> (Vec<T>, Vec<E>, usize) {
        let mut values = Vec::new();
        let mut errors = Vec::new();
        let mut empty = 0;
        for entry in self.entries {
            match entry {
                Outcome::Value(value) => values.push(value),
                Outcome::Error(error) => errors.push(error),
                Outcome::Empty => empty += 1,
            }
        }
        (values, errors, empty)
    }
}

impl<T: std::fmt::Display, E: std::fmt::Display> Ledger<T, E> {
    pub fn report(&self) -> String {
        let mut lines = Vec::new();
        for entry in &self.entries {
            match entry {
                Outcome::Value(value) => lines.push(format!("value:{value}")),
                Outcome::Error(error) => lines.push(format!("error:{error}")),
                Outcome::Empty => lines.push(String::from("empty")),
            }
        }
        let mut out = String::new();
        for line in lines {
            out.push_str(&line);
            out.push('\n');
        }
        out
    }
}
