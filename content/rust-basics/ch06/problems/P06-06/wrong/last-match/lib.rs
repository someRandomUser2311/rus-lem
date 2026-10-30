pub fn locate(text: &str, needle: &str) -> Option<usize> {
    text.rfind(needle)
}
