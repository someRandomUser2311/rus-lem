use crate::{Coin, Item};

#[derive(Debug, Clone, PartialEq)]
pub struct Machine {
    pub stock: [u32; 3],
    pub credit: u32,
}

fn change_for(credit: u32, item: Item) -> Option<u32> {
    credit.checked_sub(item.price())
}

impl Machine {
    pub fn new(stock: [u32; 3]) -> Self {
        Self { stock, credit: 0 }
    }
    pub fn insert(&mut self, coin: Coin) -> Option<u32> {
        let total = self.credit.checked_add(coin.cents());
        if let Some(n) = total {
            self.credit = n;
        }
        total
    }
    pub fn buy(&mut self, item: Item) -> Option<u32> {
        let index = item.index();
        if self.stock[index] == 0 {
            return None;
        }
        let Some(change) = change_for(self.credit, item) else {
            return None;
        };
        self.stock[index] -= 1;
        self.credit = 0;
        Some(change)
    }
    pub fn refund(&mut self) -> u32 {
        let n = self.credit;
        self.credit = 0;
        n
    }
}
