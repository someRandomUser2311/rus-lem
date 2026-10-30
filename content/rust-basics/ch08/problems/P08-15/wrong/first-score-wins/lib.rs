use std::collections::HashMap;

pub fn report(entries: Vec<(String, u32)>) -> String {
    let mut scores = HashMap::new();
    for (name, score) in entries {
        scores.entry(name).or_insert(score);
    }
    let mut rows = Vec::new();
    for (name, score) in &scores {
        rows.push((name, *score));
    }
    rows.sort();
    let mut out = String::new();
    for (name, score) in rows {
        out.push_str(&format!("{name}={score}\n"));
    }
    out
}
