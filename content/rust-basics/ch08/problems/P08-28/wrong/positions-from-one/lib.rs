use std::collections::HashMap;

pub fn positions(text: &str) -> Vec<(String, Vec<usize>)> {
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    let mut i = 0;
    for word in text.split_whitespace() {
        i += 1;
        groups.entry(word.to_string()).or_default().push(i);
    }
    let mut out = Vec::new();
    for (word, list) in groups {
        out.push((word, list));
    }
    out.sort();
    out
}
