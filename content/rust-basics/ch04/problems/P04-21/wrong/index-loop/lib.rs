pub fn slice_sum(values: &[i32]) -> i64 {
    let mut sum = 0_i64;
    for i in 0..values.len() {
        sum += values[i] as i64;
    }
    sum
}
