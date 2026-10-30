pub fn window(text: &str, start: usize, end: usize) -> &str {
    let part = &text[start..end];
    for (i, _) in text.char_indices() {
        if text[i..].starts_with(part) {
            return &text[i..i + part.len()];
        }
    }
    part
}
