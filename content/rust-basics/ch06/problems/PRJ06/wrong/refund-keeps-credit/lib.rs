#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Tea,
    Coffee,
    Water,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coin {
    Nickel,
    Dime,
    Quarter,
    Dollar,
}
impl Item {
    pub fn price(&self) -> u32 {
        match self {
            Item::Tea => 75,
            Item::Coffee => 100,
            Item::Water => 50,
        }
    }
    pub fn index(&self) -> usize {
        match self {
            Item::Tea => 0,
            Item::Coffee => 1,
            Item::Water => 2,
        }
    }
}
impl Coin {
    pub fn cents(&self) -> u32 {
        match self {
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
            Coin::Dollar => 100,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Machine {
    pub stock: [u32; 3],
    pub credit: u32,
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
        let Some(change) = self.credit.checked_sub(item.price()) else {
            return None;
        };
        self.stock[index] -= 1;
        self.credit = 0;
        Some(change)
    }
    pub fn refund(&mut self) -> u32 {
        self.credit
    }
}
