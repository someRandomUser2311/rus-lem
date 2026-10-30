pub fn normalize(text: &str) -> String {
    let mut out = String::new();
    for word in text.split_whitespace() {
        out.push_str(word);
        out.push(' ');
    }
    out
}
