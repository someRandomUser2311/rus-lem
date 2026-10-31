#[derive(Debug, Clone, PartialEq)]
pub enum DigitError {
    Length,
    NonAsciiDigit,
}

pub fn digits(text: &str) -> Result<u32, DigitError> {
    if text.is_empty() || text.chars().count() > 9 {
        return Err(DigitError::Length);
    }
    for c in text.chars() {
        if !c.is_numeric() {
            return Err(DigitError::NonAsciiDigit);
        }
    }
    Ok(text.parse::<u32>().expect("checked"))
}
