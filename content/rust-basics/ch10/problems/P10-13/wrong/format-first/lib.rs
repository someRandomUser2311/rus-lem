use std::fmt;

pub struct Label(pub String);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = format!("[{}]", self.0);
        write!(f, "{text}")
    }
}
