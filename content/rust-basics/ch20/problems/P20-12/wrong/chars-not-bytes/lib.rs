use std::fmt::Display;

pub fn describe<T: Display + ?Sized>(value: &T) -> String {
    format!("{value} ({} bytes)", std::mem::size_of_val(value))
}

pub fn widest(parts: &[&str]) -> usize {
    parts
        .iter()
        .map(|p| p.chars().count())
        .max()
        .unwrap_or(0)
}
