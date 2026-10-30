pub fn counts(text: &str) -> (usize, usize, usize) {
    (text.len(), text.chars().count(), text.chars().filter(|c| c.is_ascii()).count())
}
