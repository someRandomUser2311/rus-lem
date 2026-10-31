pub fn longest_word<'a>(a: &'a str, b: &'a str) -> &'a str {
    let mut best = &a[..0];
    let mut best_count = 0;
    for text in [a, b] {
        for word in text.split_whitespace() {
            let count = word.len();
            if count > best_count {
                best = word;
                best_count = count;
            }
        }
    }
    best
}
