pub fn sum_pair(a: Option<i32>, b: Option<i32>) -> Option<i64> {
    let Some(a) = a else {
        return None;
    };
    let Some(b) = b else {
        return None;
    };
    Some((a + b) as i64)
}
