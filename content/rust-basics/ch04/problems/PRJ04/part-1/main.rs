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
    &text[..0]
}
pub fn longest_word(text: &str) -> &str {
    &text[..0]
}
pub fn text_report(_text: &str) -> String {
    String::new()
}
fn main() {}
