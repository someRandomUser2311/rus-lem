#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Tea,
    Coffee,
    Water,
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
