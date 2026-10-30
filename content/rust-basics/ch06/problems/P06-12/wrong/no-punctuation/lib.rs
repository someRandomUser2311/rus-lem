pub fn greet(name: Option<&str>) -> String {
    if let Some(name) = name {
        format!("Hello {name}!")
    } else {
        "Hello!".to_string()
    }
}
