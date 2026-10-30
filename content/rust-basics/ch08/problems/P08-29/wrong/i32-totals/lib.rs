use std::collections::HashMap;

pub fn reconcile(changes: Vec<(String, i32)>) -> Vec<(String, i64)> {
    let mut totals = HashMap::new();
    for (name, change) in changes {
        let t = totals.entry(name).or_insert(0i64);
        *t = (*t as i32).wrapping_add(change) as i64;
    }
    let mut out = Vec::new();
    for (name, total) in totals {
        out.push((name, total));
    }
    out.sort();
    out
}
