pub struct Score {
    value: u32,
    max: u32,
}

impl Score {
    pub fn new(max: u32) -> Score {
        Score { value: 0, max }
    }

    pub fn add(&mut self, points: u32) {
        self.value = self.value.saturating_add(points);
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
