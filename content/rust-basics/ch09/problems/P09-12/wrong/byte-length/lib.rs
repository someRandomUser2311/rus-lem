#[derive(Debug, Clone, PartialEq)]
pub enum PasswordError {
    TooShort(usize),
    Whitespace,
}

#[derive(Debug, PartialEq)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(text: &str) -> Result<Password, PasswordError> {
        if text.len() < 4 {
            return Err(PasswordError::TooShort(text.len()));
        }
        for c in text.chars() {
            if c.is_whitespace() {
                return Err(PasswordError::Whitespace);
            }
        }
        Ok(Password { value: text.to_string() })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
