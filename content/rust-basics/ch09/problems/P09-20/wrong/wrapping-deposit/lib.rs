#[derive(Debug, Clone, PartialEq)]
pub enum BalanceError {
    Overflow,
    Insufficient,
}

#[derive(Debug, PartialEq)]
pub struct Balance {
    cents: u32,
}

impl Balance {
    pub fn new(cents: u32) -> Balance {
        Balance { cents }
    }

    pub fn value(&self) -> u32 {
        self.cents
    }

    pub fn deposit(&mut self, n: u32) -> Result<(), BalanceError> {
        self.cents = self.cents.wrapping_add(n);
        Ok(())
    }

    pub fn withdraw(&mut self, n: u32) -> Result<(), BalanceError> {
        self.cents = self
            .cents
            .checked_sub(n)
            .ok_or(BalanceError::Insufficient)?;
        Ok(())
    }

    pub fn transfer(&mut self, n: u32, to: &mut Balance) -> Result<(), BalanceError> {
        let from = self
            .cents
            .checked_sub(n)
            .ok_or(BalanceError::Insufficient)?;
        let into = to.cents.checked_add(n).ok_or(BalanceError::Overflow)?;
        self.cents = from;
        to.cents = into;
        Ok(())
    }
}
