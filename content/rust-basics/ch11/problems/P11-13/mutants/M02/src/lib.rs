pub fn normalize(text: &str) -> String {
    let mut words = Vec::new();
    for word in text.split_whitespace() {
        words.push(word);
    }
    words.join(" ")
}

pub fn matches(text: &str, query: &str) -> bool {
    let text = text.to_lowercase();
    let query = query.to_lowercase();
    let mut any_words = false;
    for word in query.split_whitespace() {
        any_words = true;
        if text.contains(word) {
            return true;
        }
    }
    !any_words
}
