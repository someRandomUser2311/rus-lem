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
}

pub struct Override(pub String);

impl Greet for Override {
    fn name(&self) -> String {
        self.0.clone()
    }

    fn greeting(&self) -> String {
        format!("Override: {}", self.0)
    }
}

pub fn greet_pair(a: &impl Greet, b: &impl Greet) -> String {
    format!("Hello, {}! / Hello, {}!", a.name(), b.name())
}
