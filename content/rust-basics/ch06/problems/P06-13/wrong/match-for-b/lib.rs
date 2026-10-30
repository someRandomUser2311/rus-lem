pub fn sum_pair(a: Option<i32>, b: Option<i32>) -> Option<i64> {
    let Some(a) = a else {
        return None;
    };
    let b = match b {
        Some(b) => b,
        None => return None,
    };
    Some(a as i64 + b as i64)
}
