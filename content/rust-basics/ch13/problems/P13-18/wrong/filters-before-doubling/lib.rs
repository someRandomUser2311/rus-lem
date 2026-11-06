pub fn selected(values: &[i32], threshold: i64) -> Vec<i64> {
    values
        .iter()
        .filter(|n| **n as i64 > threshold)
        .map(|n| *n as i64 * 2)
        .collect()
}
