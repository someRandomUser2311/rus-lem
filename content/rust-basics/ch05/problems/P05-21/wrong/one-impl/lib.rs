#[derive(Debug, Clone, PartialEq)]
pub struct Ledger {
    pub used: [bool; 8],
    pub names: [u32; 8],
}
impl Ledger {
    pub fn new() -> Self {
        Self {
            used: [false; 8],
            names: [0; 8],
        }
    }
    pub fn reserve(&mut self, seat: usize, id: u32) -> bool {
        if seat >= 8 || self.used[seat] {
            return false;
        }
        self.used[seat] = true;
        self.names[seat] = id;
        true
    }
    pub fn cancel(&mut self, seat: usize) -> bool {
        if seat >= 8 || !self.used[seat] {
            return false;
        }
        self.used[seat] = false;
        self.names[seat] = 0;
        true
    }
    pub fn occupied(&self) -> usize {
        let mut n = 0;
        for b in self.used {
            if b {
                n += 1;
            }
        }
        n
    }
    pub fn transfer(&mut self, from: usize, to: usize) -> bool {
        if from >= 8 || to >= 8 || !self.used[from] || self.used[to] {
            return false;
        }
        self.used[to] = true;
        self.names[to] = self.names[from];
        self.used[from] = false;
        self.names[from] = 0;
        true
    }
}
