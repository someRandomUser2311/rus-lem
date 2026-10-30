use std::collections::HashMap;

pub fn frequencies(text: &str) -> Vec<(String, usize)> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    let mut out = Vec::new();
    for (word, n) in counts {
        out.push((word, n));
    }
    out.sort();
    out
}
