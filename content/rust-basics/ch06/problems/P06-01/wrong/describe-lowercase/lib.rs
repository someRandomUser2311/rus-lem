#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    Red,
    Yellow,
    Green,
}
impl Signal {
    pub fn describe(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
pub fn signal(code: u8) -> Signal {
    if code == 0 {
        Signal::Red
    } else if code == 1 {
        Signal::Yellow
    } else {
        Signal::Green
    }
}
