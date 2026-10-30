pub fn text_counts(text: &str) -> (usize, usize, usize) {
    let mut scalars = 0;
    let mut words = 0;
    for _ in text.chars() {
        scalars += 1;
    }
    for _ in text.split_whitespace() {
        words += 1;
    }
    (text.len(), scalars, words)
}
pub fn first_word(text: &str) -> &str {
    for word in text.split_whitespace() {
        return word;
    }
    &text[..0]
}
pub fn longest_word(text: &str) -> &str {
    let mut best = &text[..0];
    let mut count = 0;
    for word in text.split_whitespace() {
        let n = word.chars().count();
        if n > count {
            best = word;
            count = n;
        }
    }
    best
}
pub fn text_report(text: &str) -> String {
    let bytes = text.len();
    let scalars = text.chars().count();
    let words = text.split_whitespace().count();
    let first = first_word(text);
    let longest = longest_word(text);
    let report = format!(
        "Bytes: {bytes}\nScalars: {scalars}\nWords: {words}\nFirst: {first}\nLongest: {longest}\n"
    );
    report
}
