#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    Red,
    Yellow,
    Green,
}
impl Signal {
    pub fn describe(&self) -> String {
        format!("{:?}", self)
    }
}
pub fn signal(code: u8) -> Signal {
    if code == 0 || code == 255 {
        Signal::Red
    } else if code == 1 {
        Signal::Yellow
    } else {
        Signal::Green
    }
}
