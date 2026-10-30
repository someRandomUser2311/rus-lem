use std::collections::{HashMap, HashSet};

pub fn catalogue(text: &str) -> Vec<(String, Vec<String>)> {
    let mut groups: HashMap<String, HashSet<String>> = HashMap::new();
    for line in text.lines() {
        let mut colon = None;
        for (i, c) in line.char_indices() {
            if c == ':' {
                colon = Some(i);
                break;
            }
        }
        if let Some(i) = colon {
            let key = line[..i].trim();
            let item = line[i + 1..].trim();
            if !key.is_empty() {
                groups
                    .entry(key.to_string())
                    .or_default()
                    .insert(item.to_string());
            }
        }
    }
    let mut out = Vec::new();
    for (key, items) in groups {
        let mut list = Vec::new();
        for item in items {
            list.push(item);
        }
        list.sort();
        out.push((key, list));
    }
    out.sort();
    out
}
