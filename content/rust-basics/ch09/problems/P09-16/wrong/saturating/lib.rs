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
    let mut result: u32 = 1;
    for token in text.split_whitespace() {
        let n = token_value(token)?;
        result = result.saturating_mul(n);
    }
    Ok(result)
}
