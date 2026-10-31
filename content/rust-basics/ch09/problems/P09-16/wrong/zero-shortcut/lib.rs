#[derive(Debug, Clone, PartialEq)]
pub enum NumberError {
    Invalid(String),
    Overflow,
}

pub fn token_value(token: &str) -> Result<u32, NumberError> {
    match token.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(NumberError::Invalid(token.to_string())),
    }
}

pub fn product(text: &str) -> Result<u32, NumberError> {
    for token in text.split_whitespace() {
        if token == "0" {
            return Ok(0);
        }
    }
    let mut result: u32 = 1;
    for token in text.split_whitespace() {
        let n = token_value(token)?;
        result = result.checked_mul(n).ok_or(NumberError::Overflow)?;
    }
    Ok(result)
}
