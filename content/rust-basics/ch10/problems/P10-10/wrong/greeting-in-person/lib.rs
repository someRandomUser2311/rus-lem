pub trait Greet {
    fn name(&self) -> String;

    fn greeting(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

pub struct Person(pub String);

impl Greet for Person {
    fn name(&self) -> String {
        self.0.clone()
    }

    fn greeting(&self) -> String {
        format!("Hello, {}!", self.0)
    }
}
