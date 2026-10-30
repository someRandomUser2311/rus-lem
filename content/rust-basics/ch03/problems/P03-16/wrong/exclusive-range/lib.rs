pub fn sum_range(n: u32) -> u64 {
    let mut total = 0_u64;
    for value in 1..n {
        total += value as u64;
    }
    total
}
