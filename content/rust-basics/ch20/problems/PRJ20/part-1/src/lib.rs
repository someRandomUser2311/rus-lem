use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Length(pub i64);

impl Add for Length {
    type Output = Length;
    fn add(self, other: Length) -> Length {
        Length(self.0.saturating_add(other.0))
    }
}

impl Sub for Length {
    type Output = Length;
    fn sub(self, other: Length) -> Length {
        Length(self.0.saturating_sub(other.0))
    }
}

impl Mul<i64> for Length {
    type Output = Length;
    fn mul(self, k: i64) -> Length {
        Length(self.0.saturating_mul(k))
    }
}

impl Neg for Length {
    type Output = Length;
    fn neg(self) -> Length {
        Length(self.0.saturating_neg())
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let mm = self.0.unsigned_abs();
        write!(f, "{sign}{}.{:03} m", mm / 1000, mm % 1000)
    }
}

pub type Parsed<T> = Result<T, String>;

pub fn parse_length(text: &str) -> Parsed<Length> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let [n, unit] = words.as_slice() else {
        return Err(String::from("expected NUMBER UNIT"));
    };
    let n: i64 = n.parse().map_err(|_| format!("bad number: {n}"))?;
    let factor: i64 = match *unit {
        "mm" => 1,
        "cm" => 10,
        "m" => 1000,
        "km" => 1_000_000,
        other => return Err(format!("unknown unit: {other}")),
    };
    n.checked_mul(factor)
        .map(Length)
        .ok_or(format!("too large: {}", text.trim()))
}

pub fn sum_text(text: &str) -> Parsed<Length> {
    let mut total = Length(0);
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let length = parse_length(line).map_err(|e| format!("line {}: {e}", i + 1))?;
        total = total + length;
    }
    Ok(total)
}

#[macro_export]
macro_rules! len {
    ($n:literal mm) => {
        $crate::Length($n)
    };
    ($n:literal cm) => {
        $crate::Length($n * 10)
    };
    ($n:literal m) => {
        $crate::Length($n * 1000)
    };
    ($n:literal km) => {
        $crate::Length($n * 1_000_000)
    };
}

#[macro_export]
macro_rules! total {
    () => {
        $crate::Length(0)
    };
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $first $(+ $rest)*
    };
}
