#[derive(Debug, Clone, PartialEq)]
pub struct BankAccount {
    pub owner: String,
    pub balance_cents: u64,
}
impl BankAccount {
    pub fn new(owner: &str) -> Self {
        Self {
            owner: owner.trim().to_string(),
            balance_cents: 0,
        }
    }
    pub fn deposit(&mut self, cents: u64) -> bool {
        false
    }
    pub fn withdraw(&mut self, cents: u64) -> bool {
        false
    }
    pub fn transfer_to(&mut self, other: &mut BankAccount, cents: u64) -> bool {
        false
    }
    pub fn summary(&self) -> String {
        format!(
            "{}: ${}.{}",
            self.owner,
            self.balance_cents / 100,
            self.balance_cents % 100
        )
    }
    pub fn close(self) -> (String, u64) {
        (self.owner, self.balance_cents)
    }
}
