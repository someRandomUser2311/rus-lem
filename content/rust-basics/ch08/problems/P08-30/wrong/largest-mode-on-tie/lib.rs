use std::collections::HashMap;

pub fn statistics(mut v: Vec<i32>) -> Option<(i32, i32)> {
    if v.is_empty() {
        return None;
    }
    v.sort();
    let median = v[(v.len() - 1) / 2];
    let mut counts = HashMap::new();
    for n in &v {
        *counts.entry(*n).or_insert(0) += 1;
    }
    let mut best = v[0];
    let mut best_count = 0;
    for (n, c) in counts {
        if c > best_count || (c == best_count && n > best) {
            best = n;
            best_count = c;
        }
    }
    Some((median, best))
}
