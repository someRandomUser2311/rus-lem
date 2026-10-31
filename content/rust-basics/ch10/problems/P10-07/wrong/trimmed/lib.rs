pub trait Named {
    fn name(&self) -> String;
}

pub struct Label(pub String);

impl Named for Label {
    fn name(&self) -> String {
        self.0.trim().to_string()
    }
}
