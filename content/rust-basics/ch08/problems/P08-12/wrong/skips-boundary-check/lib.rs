pub fn window(text: &str, start: usize, end: usize) -> Option<&str> {
    if start <= end
        && end <= text.len()

    {
        Some(&text[start..end])
    } else {
        None
    }
}
