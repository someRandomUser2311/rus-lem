#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    Empty,
    Count(u32),
    Text(String),
}
impl Packet {
    pub fn text(text: &str) -> Self {
        if text.is_empty() { Packet::Empty } else { Packet::Text(text.to_string()) }
    }
    pub fn count(n: u32) -> Self {
        Packet::Count(n)
    }
    pub fn report(&self) -> String {
        format!("{:?}", self)
    }
}
