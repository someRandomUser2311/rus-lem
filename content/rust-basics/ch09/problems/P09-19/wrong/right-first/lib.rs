#[derive(Debug, Clone, PartialEq)]
pub enum NumberError {
    Invalid(String),
    ZeroDivisor,
    Overflow,
}

pub fn operand(text: &str) -> Result<i32, NumberError> {
    match text.trim().parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(NumberError::Invalid(text.to_string())),
    }
}

pub fn quotient(a: &str, b: &str) -> Result<i32, NumberError> {
    let right = operand(b)?;
    if right == 0 {
        return Err(NumberError::ZeroDivisor);
    }
    let left = operand(a)?;
    left.checked_div(right).ok_or(NumberError::Overflow)
}
