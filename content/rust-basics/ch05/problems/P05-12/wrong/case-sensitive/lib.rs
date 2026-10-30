#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
    pub item: String,
    pub count: u32,
    pub max: u32,
}

impl Slot {
    pub fn add(&mut self, n: u32) -> u32 {
        let fit = n.min(self.max - self.count);
        self.count += fit;
        n - fit
    }

    pub fn take(&mut self, n: u32) -> u32 {
        let taken = n.min(self.count);
        self.count -= taken;
        taken
    }

    pub fn is_full(&self) -> bool {
        self.count == self.max
    }

    pub fn same_item(&self, other: &Slot) -> bool {
        self.item == other.item
    }

    pub fn label(&self) -> String {
        format!("{} x{}/{}", self.item, self.count, self.max)
    }
}
