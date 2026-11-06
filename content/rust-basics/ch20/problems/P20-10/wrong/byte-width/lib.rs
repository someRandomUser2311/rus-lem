use std::fmt;

pub trait OutlinePrint: fmt::Display {
    fn outline(&self) -> String {
        let text = self.to_string();
        let len = text.len();
        let edge = "*".repeat(len + 4);
        let blank = format!("*{}*", " ".repeat(len + 2));
        [edge.clone(), blank.clone(), format!("* {text} *"), blank, edge].join("\n")
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl OutlinePrint for Wrapper {}
