#[derive(Debug, PartialEq)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size >= size).collect()
}
