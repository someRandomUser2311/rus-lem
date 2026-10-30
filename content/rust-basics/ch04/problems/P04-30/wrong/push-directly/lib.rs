pub fn append_tag(text: &mut String, marker: char) {
    text.push(marker);
}
pub fn transform(text: String, marker: char) -> (String, String, usize) {
    let original = text.clone();
    let mut changed = String::new();
    for word in text.split_whitespace() {
        if !changed.is_empty() {
            changed.push(' ');
        }
        changed.push_str(&word.to_lowercase());
    }
    let count = changed.chars().count();
    changed.push(marker);
    (original, changed, count)
}
