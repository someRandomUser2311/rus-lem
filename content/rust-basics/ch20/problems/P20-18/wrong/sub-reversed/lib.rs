use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Checked(pub Option<i64>);

pub type BinOp = fn(Checked, Checked) -> Checked;

impl Add for Checked {
    type Output = Checked;
    fn add(self, other: Checked) -> Checked {
        Checked(self.0.zip(other.0).and_then(|(a, b)| a.checked_add(b)))
    }
}

impl Sub for Checked {
    type Output = Checked;
    fn sub(self, other: Checked) -> Checked {
        Checked(self.0.zip(other.0).and_then(|(a, b)| b.checked_sub(a)))
    }
}

impl Mul for Checked {
    type Output = Checked;
    fn mul(self, other: Checked) -> Checked {
        Checked(self.0.zip(other.0).and_then(|(a, b)| a.checked_mul(b)))
    }
}

impl Div for Checked {
    type Output = Checked;
    fn div(self, other: Checked) -> Checked {
        Checked(self.0.zip(other.0).and_then(|(a, b)| a.checked_div(b)))
    }
}

impl fmt::Display for Checked {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(n) => write!(f, "{n}"),
            None => write!(f, "error"),
        }
    }
}

pub fn op(symbol: &str) -> Option<BinOp> {
    match symbol {
        "+" => Some(|a, b| a + b),
        "-" => Some(|a, b| a - b),
        "*" => Some(|a, b| a * b),
        "/" => Some(|a, b| a / b),
        _ => None,
    }
}

pub fn eval_rpn(text: &str) -> Result<String, String> {
    let mut stack: Vec<Checked> = Vec::new();
    for token in text.split_whitespace() {
        if let Ok(n) = token.parse::<i64>() {
            stack.push(Checked(Some(n)));
        } else if let Some(f) = op(token) {
            let (Some(b), Some(a)) = (stack.pop(), stack.pop()) else {
                return Err(String::from("stack underflow"));
            };
            stack.push(f(a, b));
        } else {
            return Err(format!("bad token: {token}"));
        }
    }
    match stack.as_slice() {
        [only] => Ok(only.to_string()),
        other => Err(format!("expected one result, found {}", other.len())),
    }
}
