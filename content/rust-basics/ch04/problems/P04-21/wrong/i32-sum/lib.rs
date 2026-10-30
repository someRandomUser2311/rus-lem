pub fn slice_sum(values: &[i32]) -> i64 {
    let mut sum = 0_i32;
    for &value in values {
        sum = sum.wrapping_add(value);
    }
    sum as i64
}
