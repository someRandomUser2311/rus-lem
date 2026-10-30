pub fn window(text: &str, start: usize, end: usize) -> &str {
    if start == end {
        return "";
    }
    &text[start..end]
}
