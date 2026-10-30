use std::collections::HashMap;

pub fn score(a: String, b: String, c: String, query: &str) -> i32 {
    let mut scores = HashMap::new();
    scores.insert(a, 10);
    scores.insert(b, 20);
    if !scores.contains_key(&c) {
        scores.insert(c, 30);
    }
    scores.get(query).copied().unwrap_or(0)
}
