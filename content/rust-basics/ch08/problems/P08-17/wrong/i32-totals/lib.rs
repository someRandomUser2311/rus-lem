pub fn prefixes(v: &[i32]) -> Vec<i64> {
    let mut out = Vec::new();
    let mut total = 0i64;
    for n in v {
        total = (total as i32).wrapping_add(*n) as i64;
        out.push(total);
    }
    out
}
