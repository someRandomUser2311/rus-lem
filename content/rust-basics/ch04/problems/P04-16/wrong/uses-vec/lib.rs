pub fn first_word(text: &str) -> &str {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() { &text[..0] } else { words[0] }
}
