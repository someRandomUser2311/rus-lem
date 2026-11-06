pub fn window_sums(values: &[i32], width: usize) -> Vec<i64> {
    values
        .windows(width)
        .map(|w| w.iter().map(|n| *n as i64).sum::<i64>())
        .collect()
}
