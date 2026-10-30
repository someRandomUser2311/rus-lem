#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    Empty,
    Count(u32),
    Text(String),
}
impl Packet {
    pub fn text(text: &str) -> Self {
        Packet::Text(text.to_string())
    }
    pub fn count(n: u32) -> Self {
        if n == 0 { Packet::Empty } else { Packet::Count(n) }
    }
    pub fn report(&self) -> String {
        format!("{:?}", self)
    }
}
