#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
pub fn movement(x: i32, y: i32) -> Message {
    Message::Move { x: y, y: x }
}
pub fn writing(text: String) -> Message {
    Message::Write(text)
}
impl Message {
    pub fn describe(&self) -> String {
        format!("{:?}", self)
    }
}
