#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
pub fn movement(x: i32, y: i32) -> Message {
    Message::Move { x, y }
}
pub fn writing(text: String) -> Message {
    Message::Write(String::new())
}
impl Message {
    pub fn describe(&self) -> String {
        format!("{:?}", self)
    }
}
