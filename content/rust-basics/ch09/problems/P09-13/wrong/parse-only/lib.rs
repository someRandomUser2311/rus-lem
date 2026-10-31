#[derive(Debug, Clone, PartialEq)]
pub enum DigitError {
    Length,
    NonAsciiDigit,
}

pub fn digits(text: &str) -> Result<u32, DigitError> {
    if text.is_empty() || text.len() > 9 {
        return Err(DigitError::Length);
    }
    match text.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(DigitError::NonAsciiDigit),
    }
}
