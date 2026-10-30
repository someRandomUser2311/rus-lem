pub fn first_word(text: &str) -> Option<&str> {
    for word in text.split_ascii_whitespace() {
        return Some(word);
    }
    None
}
