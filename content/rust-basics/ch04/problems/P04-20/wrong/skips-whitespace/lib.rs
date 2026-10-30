pub fn reverse_text(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars().rev() {
        if !c.is_whitespace() {
            out.push(c);
        }
    }
    out
}
