pub fn longest_word(text: &str) -> &str {
    let mut best = &text[..0];
    let mut length = 0;
    for word in text.split_whitespace() {
        let n = word.chars().count();
        if n >= length {
            best = word;
            length = n;
        }
    }
    best
}
