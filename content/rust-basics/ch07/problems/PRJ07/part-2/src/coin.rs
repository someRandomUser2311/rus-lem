#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coin {
    Nickel,
    Dime,
    Quarter,
    Dollar,
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
