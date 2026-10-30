pub fn sum_range(n: u32) -> u64 {
    let mut total = 0_u32;
    for value in 1..=n {
        total = total.wrapping_add(value);
    }
    total as u64
}
