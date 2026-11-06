pub fn decorate(prefix: &str, text: &str) -> String {
    let label = |value: &str| format!("{value}{prefix}");
    label(text)
}
