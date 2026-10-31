pub mod labels {
    pub trait Named {
        fn name(&self) -> String;
    }

    pub struct Label(pub String);

    impl Named for Label {
        fn name(&self) -> String {
            self.0.clone()
        }
    }
}

pub fn render(value: &labels::Label) -> String {
    value.0.clone()
}
