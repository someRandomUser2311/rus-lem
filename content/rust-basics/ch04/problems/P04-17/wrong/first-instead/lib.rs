pub fn last_word(text: &str) -> &str {
    for word in text.split_whitespace() {
        return word;
    }
    &text[..0]
}
