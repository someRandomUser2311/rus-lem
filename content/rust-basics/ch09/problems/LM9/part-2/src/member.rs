#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub premium: bool,
    pub borrowed: u8,
}
impl Member {
    pub fn new(id: u32, name: &str, premium: bool) -> Member {
        Member {
            id,
            name: name.to_string(),
            premium,
            borrowed: 0,
        }
    }
    pub fn limit(&self) -> u8 {
        if self.premium { 6 } else { 3 }
    }
    pub fn can_borrow(&self) -> bool {
        self.borrowed < self.limit()
    }
    pub fn record_borrow(&mut self) -> bool {
        if !self.can_borrow() {
            return false;
        }
        self.borrowed += 1;
        true
    }
    pub fn record_return(&mut self) {
        if self.borrowed > 0 {
            self.borrowed -= 1;
        }
    }
}
