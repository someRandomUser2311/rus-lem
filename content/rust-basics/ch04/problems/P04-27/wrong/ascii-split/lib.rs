pub fn normalize(text: &str) -> String {
    let mut out = String::new();
    for word in text.split(' ') {
        if word.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}
