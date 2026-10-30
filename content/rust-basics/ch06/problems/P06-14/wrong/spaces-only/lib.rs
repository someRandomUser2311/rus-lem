pub fn first_word(text: &str) -> Option<&str> {
    for word in text.split(' ') {
        return Some(word);
    }
    None
}
