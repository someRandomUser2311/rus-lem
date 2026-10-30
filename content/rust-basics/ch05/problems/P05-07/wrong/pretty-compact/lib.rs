#[derive(Debug)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

pub fn debug_line(r: &Rect) -> String {
    format!("{:?}", r)
}

pub fn debug_pretty(r: &Rect) -> String {
    format!("{:?}", r)
}
