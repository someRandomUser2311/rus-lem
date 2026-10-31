use std::num::IntErrorKind;

pub fn classify(text: &str) -> Result<i32, String> {
    match text.trim().parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => match e.kind() {
            IntErrorKind::Empty => Err("empty".to_string()),
            IntErrorKind::InvalidDigit => Err("digit".to_string()),
            _ => Err("too big".to_string()),
        },
    }
}
