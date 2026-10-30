#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dish {
    Soup,
    Salad,
    Rice,
}

impl Dish {
    pub fn price(&self) -> u32 {
        crate::kitchen::menu::price(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub dish: Dish,
    pub quantity: u32,
}
