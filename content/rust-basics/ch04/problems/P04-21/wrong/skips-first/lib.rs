pub fn slice_sum(values: &[i32]) -> i64 {
    let mut sum = 0_i64;
    for &value in &values[1.min(values.len())..] {
        sum += value as i64;
    }
    sum
}
