pub fn first_word(text: &str) -> &str {
    for word in text.split(' ') {
        return word;
    }
    &text[..0]
}
