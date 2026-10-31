use std::fmt;

#[derive(Debug)]
pub struct Label(pub String);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}
