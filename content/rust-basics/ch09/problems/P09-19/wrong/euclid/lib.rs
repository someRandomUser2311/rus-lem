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
    let left = operand(a)?;
    let right = operand(b)?;
    if right == 0 {
        return Err(NumberError::ZeroDivisor);
    }
    left.checked_div_euclid(right).ok_or(NumberError::Overflow)
}
