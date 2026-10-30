pub fn last_word(text: &str) -> &str {
    let mut last = &text[..0];
    for word in text.split(' ') {
        if !word.is_empty() {
            last = word;
        }
    }
    last
}
