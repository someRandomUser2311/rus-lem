pub fn decorate(text: &str) -> String {
    let decorated = format!("[{}]", text.trim());
    decorated
}
