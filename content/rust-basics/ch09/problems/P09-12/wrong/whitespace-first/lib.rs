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
        for c in text.chars() {
            if c.is_whitespace() {
                return Err(PasswordError::Whitespace);
            }
        }
        let count = text.chars().count();
        if count < 4 {
            return Err(PasswordError::TooShort(count));
        }
        Ok(Password { value: text.to_string() })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
