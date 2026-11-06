pub fn greeting(name: &str) -> String {
    format!("Hello, {name}")
}

pub fn boxed_greeting(name: Box<String>) -> String {
    greeting(&name)
}
