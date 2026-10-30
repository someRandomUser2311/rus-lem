pub fn greet(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("Hello, {name}!"),
        None => "Hello!".to_string(),
    }
}
