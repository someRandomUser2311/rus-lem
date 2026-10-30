use std::collections::HashSet;

pub fn first_repeat(text: &str) -> Option<char> {
    let mut seen = HashSet::new();
    let mut found = None;
    for c in text.chars() {
        if !seen.insert(c) {
            found = Some(c);
        }
    }
    found
}
