pub fn reverse_text(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        out.push(c);
    }
    out
}
