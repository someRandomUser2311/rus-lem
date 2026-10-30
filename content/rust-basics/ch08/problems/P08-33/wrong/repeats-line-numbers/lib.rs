use std::collections::{HashMap, HashSet};

pub fn index(text: &str) -> Vec<(String, Vec<usize>)> {
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    let mut number = 0;
    for line in text.lines() {
        number += 1;
        let mut seen = HashSet::new();
        for word in line.split_whitespace() {
            if seen.insert(word) || true {
                groups.entry(word.to_string()).or_default().push(number);
            }
        }
    }
    let mut out = Vec::new();
    for (word, lines) in groups {
        out.push((word, lines));
    }
    out.sort();
    out
}
