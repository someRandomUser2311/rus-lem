use std::collections::HashSet;

pub fn distinct(text: &str) -> usize {
    let mut seen = HashSet::new();
    for word in text.split(' ') {
        seen.insert(word);
    }
    seen.len()
}
