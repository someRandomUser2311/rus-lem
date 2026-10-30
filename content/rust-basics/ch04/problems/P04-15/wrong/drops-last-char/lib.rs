pub fn window(text: &str, start: usize, end: usize) -> &str {
    let mut last = start;
    for (i, _) in text[start..end].char_indices() {
        last = start + i;
    }
    &text[start..last]
}
