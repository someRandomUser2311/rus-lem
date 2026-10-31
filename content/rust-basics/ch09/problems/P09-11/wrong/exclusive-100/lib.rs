#[derive(Debug, PartialEq)]
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, u32> {
        if value < 1 || value >= 100 {
            return Err(value);
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
