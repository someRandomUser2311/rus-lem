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

pub use labels::Named;

pub fn render(value: &labels::Label) -> String {
    use labels::Named;
    value.name()
}
