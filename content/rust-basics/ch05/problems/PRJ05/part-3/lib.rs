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
        if cents > u64::MAX - self.balance_cents {
            return false;
        }
        self.balance_cents += cents;
        true
    }
    pub fn withdraw(&mut self, cents: u64) -> bool {
        if cents > self.balance_cents {
            return false;
        }
        self.balance_cents -= cents;
        true
    }
    pub fn transfer_to(&mut self, other: &mut BankAccount, cents: u64) -> bool {
        if cents > self.balance_cents || cents > u64::MAX - other.balance_cents {
            return false;
        }
        self.balance_cents -= cents;
        other.balance_cents += cents;
        true
    }
    pub fn summary(&self) -> String {
        format!(
            "{}: ${}.{:02}",
            self.owner,
            self.balance_cents / 100,
            self.balance_cents % 100
        )
    }
    pub fn close(self) -> (String, u64) {
        (self.owner, self.balance_cents)
    }
}
