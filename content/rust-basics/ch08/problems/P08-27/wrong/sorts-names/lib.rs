use std::collections::HashSet;

pub fn stable_unique(names: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for name in names {
        if seen.insert(name.clone()) {
            out.push(name);
        }
    }
    out.sort();
    out
}
