pub trait Named {
    fn name(&self) -> String;
}

pub struct Label(pub String);

impl Named for Label {
    fn name(&self) -> String {
        self.0.clone()
    }
}

pub fn make_named(text: &str) -> impl Named + use<> {
    Label(text.to_string())
}
